use axum::{extract::State, response::Json};
use serde_json::{json, Value};
use time::OffsetDateTime;

use crate::config::Config;

pub async fn health(State(_config): State<Config>) -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": OffsetDateTime::now_utc()
    }))
}