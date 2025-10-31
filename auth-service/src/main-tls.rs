// TLS-enabled main.rs für auth-service
use anyhow::{Context, Result};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
};
use tracing::{info, warn};

mod config;
mod handlers;
mod jwt;
mod logging;
mod middleware as custom_middleware;
mod models;
mod password;
mod storage;
mod tls;

use config::Config;
use storage::FileStorage;
use tls::{TlsConfig, TlsManager};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::setup_logging(false);

    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;

    info!(
        service = "auth-service",
        version = env!("CARGO_PKG_VERSION"),
        "Starting UM-OIC Auth Service with native TLS"
    );

    // Initialize storage
    let storage = Arc::new(RwLock::new(
        FileStorage::new(&config.data_dir)
            .await
            .context("Failed to initialize storage")?,
    ));

    // Initialize JWT service
    let jwt_service = Arc::new(
        jwt::JwtService::new(&config.jwt.secret)
            .context("Failed to initialize JWT service")?,
    );

    // Application state
    let app_state = (storage, jwt_service, config.clone());

    // Build application router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health))

        // Authentication routes
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/logout", post(handlers::auth::logout))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/reset-password", post(handlers::auth::reset_password))
        .route("/auth/refresh", post(handlers::auth::refresh))

        // OAuth2/OIDC routes
        .route("/oauth2/authorize", get(handlers::oauth::authorize))
        .route("/oauth2/token", post(handlers::oauth::token))
        .route("/oauth2/userinfo", get(handlers::oauth::userinfo))
        .route("/.well-known/openid_configuration", get(handlers::oauth::discovery))

        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn(custom_middleware::security::security_headers))
        )
        .with_state(app_state);

    // Setup TLS
    let tls_config = TlsConfig {
        cert_path: std::env::var("TLS_CERT_PATH")
            .unwrap_or_else(|_| "/app/certs/cert.pem".to_string()),
        key_path: std::env::var("TLS_KEY_PATH")
            .unwrap_or_else(|_| "/app/certs/key.pem".to_string()),
        auto_reload: std::env::var("TLS_AUTO_RELOAD")
            .map(|v| v.parse().unwrap_or(true))
            .unwrap_or(true),
        acme_enabled: std::env::var("ACME_ENABLED")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false),
        acme_email: std::env::var("ACME_EMAIL").ok(),
        domain: std::env::var("DOMAIN")
            .unwrap_or_else(|_| "localhost".to_string()),
    };

    let tls_manager = TlsManager::new(tls_config.clone());

    // Setup automatic certificate renewal
    if tls_config.acme_enabled {
        tls_manager.start_auto_renewal().await?;
    }

    // Create RustLS config
    let rustls_config = tls_manager.create_rustls_config().await?;

    // Determine bind address
    let bind_addr: SocketAddr = format!(
        "{}:{}",
        std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string()),
        config.server.port
    ).parse().context("Invalid bind address")?;

    info!(
        "Auth service starting with TLS on https://{}",
        bind_addr
    );

    // Start HTTPS server
    axum_server::bind_rustls(bind_addr, rustls_config)
        .serve(app.into_make_service())
        .await
        .context("Server failed")?;

    Ok(())
}

/// Alternativer Start für HTTP + HTTPS (beide Ports)
#[allow(dead_code)]
async fn start_dual_mode(app: Router, config: &Config) -> Result<()> {
    let tls_config = TlsConfig::default();
    let tls_manager = TlsManager::new(tls_config);
    let rustls_config = tls_manager.create_rustls_config().await?;

    // HTTP Server (Port 8000)
    let http_addr: SocketAddr = format!("0.0.0.0:{}", config.server.port)
        .parse()
        .context("Invalid HTTP address")?;

    // HTTPS Server (Port 8443)
    let https_addr: SocketAddr = format!("0.0.0.0:8443")
        .parse()
        .context("Invalid HTTPS address")?;

    info!("Starting dual mode - HTTP: {}, HTTPS: {}", http_addr, https_addr);

    // Start both servers concurrently
    let http_server = axum_server::bind(http_addr)
        .serve(app.clone().into_make_service());

    let https_server = axum_server::bind_rustls(https_addr, rustls_config)
        .serve(app.into_make_service());

    tokio::try_join!(http_server, https_server)?;

    Ok(())
}

/// HTTP zu HTTPS Redirect Server
#[allow(dead_code)]
async fn start_redirect_server(port: u16) -> Result<()> {
    use axum::http::{StatusCode, Uri};
    use axum::response::Redirect;

    let redirect_app = Router::new()
        .fallback(|uri: Uri| async move {
            let https_uri = format!("https://localhost:8443{}", uri.path_and_query().map_or("", |pq| pq.as_str()));
            Redirect::permanent(&https_uri)
        });

    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;

    info!("Starting HTTP to HTTPS redirect server on {}", addr);

    axum_server::bind(addr)
        .serve(redirect_app.into_make_service())
        .await?;

    Ok(())
}