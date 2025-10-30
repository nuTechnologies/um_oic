use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router,
};
use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, signal::unix::{signal, SignalKind}, sync::RwLock};
use tower_http::services::ServeDir;
use tracing::{info, error};

mod config;
mod storage;
mod models;
mod handlers;
mod middleware;
mod logging;
mod jwt;
mod password;

use config::Config;
use storage::FileStorage;

#[derive(Parser)]
#[command(name = "auth-service")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Data directory path
    #[arg(long, env = "AUTH_DATA_DIR", default_value = "./data")]
    data_dir: String,

    /// Certificate directory (key.pem, cert.pem)
    #[arg(long, env = "AUTH_CERT_DIR", default_value = "./certs")]
    cert_dir: String,

    /// Enable debug logging
    #[arg(long, env = "AUTH_DEBUG")]
    debug: bool,

    /// Configuration file
    #[arg(long, env = "AUTH_CONFIG", default_value = "config.toml")]
    config: String,

    /// Bind address
    #[arg(long, env = "AUTH_BIND", default_value = "0.0.0.0:8000")]
    bind: String,

    /// PID file location
    #[arg(long, env = "AUTH_PID_FILE", default_value = "/var/run/auth-service.pid")]
    pid_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    logging::setup_logging(args.debug);

    // Write PID file
    write_pid_file(&args.pid_file)?;

    // Load configuration
    let config = Config::load(&args.config).await
        .context("Failed to load configuration")?;

    // Load data storage
    let storage = Arc::new(RwLock::new(
        FileStorage::load(&args.data_dir).await
            .context("Failed to load data storage")?
    ));

    info!(
        service = "auth-service",
        event = "startup",
        bind = %args.bind,
        data_dir = %args.data_dir,
        users_loaded = storage.read().await.users_count(),
        version = env!("CARGO_PKG_VERSION")
    );

    // Setup SIGHUP handler for data reload
    setup_reload_handler(storage.clone(), args.data_dir.clone());

    // Setup graceful shutdown
    setup_shutdown_handler(args.pid_file.clone());

    // Create application router
    let app = create_app(storage, config).await?;

    // Parse bind address
    let addr: SocketAddr = args.bind.parse()
        .context("Invalid bind address")?;

    // Start server
    let listener = TcpListener::bind(&addr).await
        .context("Failed to bind listener")?;

    info!(
        service = "auth-service",
        event = "listening",
        addr = %addr
    );

    axum::serve(listener, app).await
        .context("Server error")?;

    Ok(())
}

async fn create_app(
    storage: Arc<RwLock<FileStorage>>,
    config: Config
) -> Result<Router> {
    let jwt_service = Arc::new(jwt::JwtService::new(&config.jwt_secret));

    let app = Router::new()
        // Static files (login UI, assets)
        .nest_service("/", ServeDir::new("data/web/auth"))

        // Authentication API
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/auth/forgot-password", post(handlers::auth::forgot_password))
        .route("/api/auth/reset-password", post(handlers::auth::reset_password))

        // OAuth2/OIDC endpoints
        .route("/oauth2/authorize", get(handlers::oauth::authorize))
        .route("/oauth2/token", post(handlers::oauth::token))
        .route("/oauth2/userinfo", get(handlers::oauth::userinfo))
        .route("/.well-known/openid-configuration", get(handlers::oauth::discovery))

        // Health check
        .route("/health", get(handlers::health::health))

        // Security middleware
        .layer(axum::middleware::from_fn(middleware::security::security_headers))

        // Shared state
        .with_state((storage, jwt_service, config));

    Ok(app)
}

fn write_pid_file(pid_file: &str) -> Result<()> {
    let pid = std::process::id();
    std::fs::write(pid_file, pid.to_string())
        .context("Failed to write PID file")?;

    info!(
        service = "auth-service",
        event = "pid_written",
        pid = pid,
        pid_file = pid_file
    );

    Ok(())
}

fn setup_reload_handler(storage: Arc<RwLock<FileStorage>>, data_dir: String) {
    tokio::spawn(async move {
        let mut sighup = signal(SignalKind::hangup()).expect("Failed to setup SIGHUP handler");

        loop {
            sighup.recv().await;

            info!(
                service = "auth-service",
                event = "reload_triggered",
                trigger = "sighup"
            );

            let start_time = std::time::Instant::now();

            match FileStorage::load(&data_dir).await {
                Ok(new_storage) => {
                    let users_count = new_storage.users_count();
                    let groups_count = new_storage.groups_count();

                    {
                        let mut storage_guard = storage.write().await;
                        *storage_guard = new_storage;
                    }

                    info!(
                        service = "auth-service",
                        event = "data_reloaded",
                        trigger = "sighup",
                        users_count = users_count,
                        groups_count = groups_count,
                        duration_ms = start_time.elapsed().as_millis()
                    );
                }
                Err(e) => {
                    error!(
                        service = "auth-service",
                        event = "reload_failed",
                        trigger = "sighup",
                        error = %e,
                        duration_ms = start_time.elapsed().as_millis()
                    );
                }
            }
        }
    });
}

fn setup_shutdown_handler(pid_file: String) {
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to setup SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                info!(service = "auth-service", event = "shutdown", signal = "SIGTERM");
            }
            _ = sigint.recv() => {
                info!(service = "auth-service", event = "shutdown", signal = "SIGINT");
            }
        }

        // Cleanup PID file
        if let Err(e) = std::fs::remove_file(&pid_file) {
            error!(
                service = "auth-service",
                event = "pid_cleanup_failed",
                error = %e,
                pid_file = %pid_file
            );
        }

        std::process::exit(0);
    });
}