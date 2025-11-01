use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{
    config::Config,
    jwt::JwtVerifier,
    models::{Claims, UserResponse},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn list(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Value>>, StatusCode> {
    let _storage_guard = storage.read().await;

    // Return placeholder organizations
    let organizations = vec![
        json!({
            "id": "default",
            "name": "Default Organization",
            "domain": "default.local"
        }),
    ];

    info!(
        service = "admin-service",
        event = "organizations_listed",
        requested_by = %claims.sub,
        count = organizations.len()
    );

    Ok(Json(organizations))
}

pub async fn list_users(
    Path(org): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let storage_guard = storage.read().await;

    // Get users for the organization
    let users: Vec<UserResponse> = storage_guard
        .get_all_users()
        .filter(|u| u.org == org)
        .cloned()
        .map(UserResponse::from)
        .collect();

    info!(
        service = "admin-service",
        event = "organization_users_listed",
        org = %org,
        requested_by = %claims.sub,
        count = users.len()
    );

    Ok(Json(users))
}

pub async fn create(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_create_request",
        requested_by = %claims.sub
    );

    // For now, return success but don't actually create (organizations are managed externally)
    let response = json!({
        "success": true,
        "message": "Organization creation not implemented - organizations are managed externally"
    });

    Ok(Json(response))
}

pub async fn update(
    Path(org_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_update_request",
        org_id = %org_id,
        requested_by = %claims.sub
    );

    // For now, return success but don't actually update
    let response = json!({
        "success": true,
        "message": "Organization update not implemented - organizations are managed externally"
    });

    Ok(Json(response))
}

pub async fn delete(
    Path(org_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_delete_request",
        org_id = %org_id,
        requested_by = %claims.sub
    );

    // For now, return success but don't actually delete
    let response = json!({
        "success": true,
        "message": "Organization deletion not implemented - organizations are managed externally"
    });

    Ok(Json(response))
}