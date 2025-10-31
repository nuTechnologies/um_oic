use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{
    config::Config,
    jwt::JwtVerifier,
    models::{Claims, Role},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn list(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Role>>, StatusCode> {
    let storage_guard = storage.read().await;

    // For now, return predefined roles since roles are not fully implemented in current data model
    let roles: Vec<Role> = vec![
        Role {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            description: "Full system administration access".to_string(),
            permissions: vec!["*".to_string()],
        },
        Role {
            id: "adminread".to_string(),
            name: "Read-only Administrator".to_string(),
            description: "Read-only administration access".to_string(),
            permissions: vec!["read:*".to_string()],
        },
    ];

    info!(
        service = "admin-service",
        event = "roles_listed",
        requested_by = %claims.sub,
        count = roles.len()
    );

    Ok(Json(roles))
}

pub async fn create(
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(_payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement role creation
    info!(
        service = "admin-service",
        event = "role_create_requested",
        created_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Role creation not yet implemented"
    })))
}

pub async fn update(
    Path(role_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(_payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement role update
    info!(
        service = "admin-service",
        event = "role_update_requested",
        role_id = %role_id,
        updated_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Role update not yet implemented"
    })))
}

pub async fn delete(
    Path(role_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement role deletion
    info!(
        service = "admin-service",
        event = "role_delete_requested",
        role_id = %role_id,
        deleted_by = %claims.sub
    );

    Ok(StatusCode::NO_CONTENT)
}