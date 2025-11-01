use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    config::Config,
    jwt::JwtVerifier,
    models::{Claims, AuditEvent},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    user_id: Option<String>,
    event_type: Option<String>,
    from: Option<String>,
    to: Option<String>,
    limit: Option<u32>,
}

pub async fn query(
    Query(query): Query<AuditQuery>,
    State((storage, _jwt_verifier, _config)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    info!(
        service = "admin-service",
        event = "audit_query_requested",
        requested_by = %claims.sub,
        user_id = ?query.user_id,
        event_type = ?query.event_type,
        limit = ?query.limit
    );

    let storage_guard = storage.read().await;
    let audit_events = storage_guard.query_audit_events(
        query.user_id.as_deref(),
        query.event_type.as_deref(),
        query.from.as_deref(),
        query.to.as_deref(),
        query.limit.unwrap_or(50)
    );

    drop(storage_guard);

    let events: Vec<Value> = audit_events.into_iter()
        .map(|event| serde_json::to_value(event).unwrap_or_default())
        .collect();

    Ok(Json(events))
}