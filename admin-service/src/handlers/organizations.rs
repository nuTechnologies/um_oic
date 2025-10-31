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
use tracing::info;

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