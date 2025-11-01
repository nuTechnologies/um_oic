use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use time::OffsetDateTime;

use crate::{config::Config, storage::FileStorage};

type AppState = (Arc<RwLock<FileStorage>>, Arc<crate::jwt::JwtService>, Config);

pub async fn health(State((storage, _, _)): State<AppState>) -> Json<Value> {
    let storage_guard = storage.read().await;

    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime_seconds": get_uptime_seconds(),
        "users_loaded": storage_guard.users_count(),
        "timestamp": OffsetDateTime::now_utc()
    }))
}

fn get_uptime_seconds() -> u64 {
    // Simple uptime tracking - in production you might want to store startup time
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}