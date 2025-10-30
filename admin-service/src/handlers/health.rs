use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use time::OffsetDateTime;

use crate::{config::Config, jwt::JwtVerifier, storage::AdminStorage};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn health(State((storage, _, _)): State<AppState>) -> Json<Value> {
    let storage_guard = storage.read().await;

    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "users_loaded": storage_guard.users_count(),
        "groups_loaded": storage_guard.groups_count(),
        "roles_loaded": storage_guard.roles_count(),
        "clients_loaded": storage_guard.clients_count(),
        "auth_data_stale": storage_guard.is_auth_stale(),
        "timestamp": OffsetDateTime::now_utc()
    }))
}