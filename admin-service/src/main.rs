use anyhow::{Context, Result};
use axum::{
    routing::{get, post, patch, delete},
    Router,
    middleware::Next,
    response::Response,
    extract::Request,
    http::HeaderValue,
};
use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::services::ServeDir;
use tracing::info;

mod config;
mod storage;
mod models;
mod handlers;
mod middleware;
mod logging;
mod jwt;
mod password;
mod tls;

use config::Config;
use storage::AdminStorage;

#[derive(Parser)]
#[command(name = "admin-service")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Data directory path (read-write access)
    #[arg(long, env = "ADMIN_DATA_DIR", default_value = "./data")]
    data_dir: String,

    /// Certificate directory
    #[arg(long, env = "ADMIN_CERT_DIR", default_value = "./certs")]
    cert_dir: String,

    /// Enable debug logging
    #[arg(long, env = "ADMIN_DEBUG")]
    debug: bool,

    /// Configuration file
    #[arg(long, env = "ADMIN_CONFIG", default_value = "config.toml")]
    config: String,

    /// Bind address
    #[arg(long, env = "ADMIN_BIND", default_value = "0.0.0.0:8445")]
    bind: String,

    /// Enable TLS
    #[arg(long, env = "ADMIN_TLS_ENABLE")]
    tls_enable: bool,

    /// Auth service PID file (für SIGHUP reload trigger)
    #[arg(long, env = "AUTH_PID_FILE", default_value = "/var/run/auth-service.pid")]
    auth_pid_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    logging::setup_logging(args.debug);

    // Load configuration
    let config = Config::load(&args.config).await
        .context("Failed to load configuration")?;

    // Load data storage with write access
    let storage = Arc::new(RwLock::new(
        AdminStorage::load(&args.data_dir, &args.auth_pid_file).await
            .context("Failed to load admin storage")?
    ));

    info!(
        service = "admin-service",
        event = "startup",
        bind = %args.bind,
        data_dir = %args.data_dir,
        users_loaded = storage.read().await.users_count(),
        version = env!("CARGO_PKG_VERSION")
    );

    // Setup graceful shutdown
    setup_shutdown_handler();

    // Create application router
    let app = create_app(storage, config).await?;

    // Parse bind address
    let addr: SocketAddr = args.bind.parse()
        .context("Invalid bind address")?;

    if args.tls_enable {
        // TLS mode
        info!(
            service = "admin-service",
            event = "startup",
            version = env!("CARGO_PKG_VERSION"),
            tls_enabled = true
        );

        let tls_config = tls::TlsConfig {
            cert_path: std::env::var("TLS_CERT_PATH").unwrap_or_else(|_| "../certs/admin-cert.pem".to_string()),
            key_path: std::env::var("TLS_KEY_PATH").unwrap_or_else(|_| "../certs/admin-key.pem".to_string()),
            domain: std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
            auto_generate: std::env::var("TLS_AUTO_GENERATE").map(|v| v == "true").unwrap_or(true),
        };

        let tls_manager = tls::TlsManager::new(tls_config);

        let rustls_config = tls_manager.create_rustls_config().await
            .context("Failed to configure TLS")?;

        info!(
            service = "admin-service",
            event = "tls_server_starting",
            addr = %addr,
            domain = "localhost"
        );

        let server = axum_server::bind_rustls(addr, rustls_config)
            .serve(app.into_make_service());

        server.await.context("TLS server error")?;
    } else {
        // HTTP mode
        let listener = TcpListener::bind(&addr).await
            .context("Failed to bind listener")?;

        info!(
            service = "admin-service",
            event = "listening",
            addr = %addr
        );

        axum::serve(listener, app).await
            .context("Server error")?;
    }

    Ok(())
}

async fn create_app(
    storage: Arc<RwLock<AdminStorage>>,
    config: Config
) -> Result<Router> {
    let jwt_verifier = Arc::new(jwt::JwtVerifier::new(&config.jwt_public_key));

    // Create API routes with authentication middleware
    let api_routes = Router::new()
        // Auth API
        .route("/api/auth/me", get(handlers::auth::me))

        // Users API
        .route("/api/users", get(handlers::users::list).post(handlers::users::create))
        .route("/api/users/:id", get(handlers::users::get).patch(handlers::users::update).delete(handlers::users::delete))
        .route("/api/users/:id/reset-password", post(handlers::users::reset_password))

        // Organizations API
        .route("/api/organizations", get(handlers::organizations::list).post(handlers::organizations::create))
        .route("/api/organizations/:id", patch(handlers::organizations::update).delete(handlers::organizations::delete))
        .route("/api/organizations/:org/users", get(handlers::organizations::list_users))


        // Clients API
        .route("/api/clients", get(handlers::clients::list).post(handlers::clients::create))
        .route("/api/clients/:id", get(handlers::clients::get).patch(handlers::clients::update).delete(handlers::clients::delete))
        .route("/api/clients/:id/rotate-secret", post(handlers::clients::rotate_secret))

        // System API
        .route("/api/system/status", get(handlers::system::status))
        .route("/api/system/stats", get(handlers::system::stats))
        .route("/api/system/reload-auth", post(handlers::system::reload_auth))

        // Audit API
        .route("/api/audit", get(handlers::audit::query))

        // Claims API
        .route("/api/claims", get(handlers::claims::list).post(handlers::claims::create))
        .route("/api/claims/:key", patch(handlers::claims::update).delete(handlers::claims::delete))

        // Admin authentication middleware for API routes only
        .layer(axum::middleware::from_fn_with_state(
            (jwt_verifier.clone(), config.clone()),
            middleware::auth::require_admin
        ));

    let app = Router::new()

        // Static files (admin UI) - no authentication required
        .nest_service("/", ServeDir::new("./data/web/mgmt"))
        .nest_service("/mgmt", ServeDir::new("./data/web/mgmt"))

        // Health check - no authentication required
        .route("/health", get(handlers::health::health))

        // Add cache-control headers to prevent stale auth data
        .layer(axum::middleware::from_fn(add_cache_headers))

        // Shared state
        .with_state(config.clone())

        // API routes with authentication (using tuple state)
        .merge(api_routes.with_state((storage, jwt_verifier, config)));

    Ok(app)
}

async fn add_cache_headers(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;

    // Add cache-control headers for API endpoints
    let headers = response.headers_mut();
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache, no-store, must-revalidate"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Expires", HeaderValue::from_static("0"));

    response
}

fn setup_shutdown_handler() {
    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to setup SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to setup SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                info!(service = "admin-service", event = "shutdown", signal = "SIGTERM");
            }
            _ = sigint.recv() => {
                info!(service = "admin-service", event = "shutdown", signal = "SIGINT");
            }
        }

        std::process::exit(0);
    });
}