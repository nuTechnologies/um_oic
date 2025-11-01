use axum::{extract::{State, Path}, http::StatusCode, response::Json, Extension};
use anyhow::Result;
use serde_json::{json, Value};
use tracing::{info, warn};

use crate::{config::Config, jwt::JwtVerifier, storage::AdminStorage, models::Claims};

pub async fn list(
    State((storage, _jwt_verifier, _config)): State<(
        std::sync::Arc<tokio::sync::RwLock<AdminStorage>>,
        std::sync::Arc<JwtVerifier>,
        Config,
    )>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claims_list_request"
    );

    let claims_registry = {
        let storage_guard = storage.read().await;
        storage_guard.get_claims().clone()
    };

    let response = serde_json::json!({
        "claims": claims_registry.claims
    });

    info!(
        service = "admin-service",
        event = "claims_list_success",
        count = claims_registry.claims.len()
    );

    Ok(Json(response))
}

type AppState = (
    std::sync::Arc<tokio::sync::RwLock<AdminStorage>>,
    std::sync::Arc<JwtVerifier>,
    Config,
);

pub async fn create(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_create_request",
        requested_by = %claims.sub
    );

    // For now, return success but don't actually create (claims are managed externally)
    let response = json!({
        "success": true,
        "message": "Claim creation not implemented - claims are managed externally"
    });

    Ok(Json(response))
}

pub async fn update(
    Path(claim_key): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_update_request",
        claim_key = %claim_key,
        requested_by = %claims.sub
    );

    // For now, return success but don't actually update
    let response = json!({
        "success": true,
        "message": "Claim update not implemented - claims are managed externally"
    });

    Ok(Json(response))
}

pub async fn delete(
    Path(claim_key): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_delete_request",
        claim_key = %claim_key,
        requested_by = %claims.sub
    );

    // For now, return success but don't actually delete
    let response = json!({
        "success": true,
        "message": "Claim deletion not implemented - claims are managed externally"
    });

    Ok(Json(response))
}