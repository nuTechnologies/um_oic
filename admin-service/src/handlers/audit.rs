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
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    // TODO: Implement audit log querying from JSONL files
    info!(
        service = "admin-service",
        event = "audit_query_requested",
        requested_by = %claims.sub,
        user_id = ?query.user_id,
        event_type = ?query.event_type,
        limit = ?query.limit
    );

    // Placeholder response
    Ok(Json(vec![
        json!({
            "id": "evt-001",
            "user_id": "user-550e8400",
            "event_type": "login",
            "ip_address": "192.168.1.100",
            "created_at": "2025-10-30T08:00:00Z"
        }),
        json!({
            "id": "evt-002",
            "user_id": "user-550e8400",
            "event_type": "user_created",
            "metadata": {
                "created_user_id": "user-660e8400"
            },
            "created_at": "2025-10-30T09:15:00Z"
        })
    ]))
}