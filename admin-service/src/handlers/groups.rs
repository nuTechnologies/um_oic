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
    models::{Claims, Group, UserResponse},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn list(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Group>>, StatusCode> {
    let storage_guard = storage.read().await;

    let groups: Vec<Group> = storage_guard.get_all_groups()
        .cloned()
        .collect();

    info!(
        service = "admin-service",
        event = "groups_listed",
        requested_by = %claims.sub,
        count = groups.len()
    );

    Ok(Json(groups))
}

pub async fn get(
    Path(group_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Group>, StatusCode> {
    let storage_guard = storage.read().await;

    let group = storage_guard.get_group(&group_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    info!(
        service = "admin-service",
        event = "group_retrieved",
        group_id = %group_id,
        requested_by = %claims.sub
    );

    Ok(Json(group.clone()))
}

pub async fn create(
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement group creation
    info!(
        service = "admin-service",
        event = "group_create_requested",
        created_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Group creation not yet implemented"
    })))
}

pub async fn update(
    Path(group_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement group update
    info!(
        service = "admin-service",
        event = "group_update_requested",
        group_id = %group_id,
        updated_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Group update not yet implemented"
    })))
}

pub async fn delete(
    Path(group_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement group deletion
    info!(
        service = "admin-service",
        event = "group_delete_requested",
        group_id = %group_id,
        deleted_by = %claims.sub
    );

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_members(
    Path(group_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let storage_guard = storage.read().await;

    // For now, return empty list since group membership is not implemented in current data model
    let members: Vec<crate::models::User> = vec![];
    let member_responses: Vec<UserResponse> = members
        .into_iter()
        .map(UserResponse::from)
        .collect();

    info!(
        service = "admin-service",
        event = "group_members_listed",
        group_id = %group_id,
        requested_by = %claims.sub,
        member_count = member_responses.len()
    );

    Ok(Json(member_responses))
}