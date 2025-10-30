use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{
    config::Config,
    jwt::JwtVerifier,
    models::{Claims, SystemStatus},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn status(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SystemStatus>, StatusCode> {
    let storage_guard = storage.read().await;

    let last_auth_reload = if storage_guard.sync_state.last_auth_reload.elapsed().unwrap().as_secs() == 0 {
        None
    } else {
        Some(OffsetDateTime::from(storage_guard.sync_state.last_auth_reload))
    };

    let status = SystemStatus {
        status: "healthy".to_string(),
        auth_data_stale: storage_guard.is_auth_stale(),
        last_auth_reload,
        last_data_update: OffsetDateTime::from(storage_guard.sync_state.last_data_update),
        users_count: storage_guard.users_count(),
        groups_count: storage_guard.groups_count(),
        roles_count: storage_guard.roles_count(),
        clients_count: storage_guard.clients_count(),
    };

    info!(
        service = "admin-service",
        event = "system_status_requested",
        requested_by = %claims.sub,
        auth_data_stale = status.auth_data_stale
    );

    Ok(Json(status))
}

pub async fn reload_auth(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    let mut storage_guard = storage.write().await;

    match storage_guard.trigger_auth_reload().await {
        Ok(()) => {
            info!(
                service = "admin-service",
                event = "auth_reload_success",
                triggered_by = %claims.sub
            );

            Ok(Json(json!({
                "success": true,
                "message": "Auth service reload triggered successfully"
            })))
        }
        Err(e) => {
            warn!(
                service = "admin-service",
                event = "auth_reload_failed",
                triggered_by = %claims.sub,
                error = %e
            );

            Ok(Json(json!({
                "success": false,
                "message": format!("Failed to trigger auth reload: {}", e)
            })))
        }
    }
}