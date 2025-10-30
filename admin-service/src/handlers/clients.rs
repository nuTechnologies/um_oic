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
    models::{Claims, Client},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn list(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Client>>, StatusCode> {
    let storage_guard = storage.read().await;

    let clients: Vec<Client> = storage_guard.get_all_clients()
        .cloned()
        .collect();

    info!(
        service = "admin-service",
        event = "clients_listed",
        requested_by = %claims.sub,
        count = clients.len()
    );

    Ok(Json(clients))
}

pub async fn get(
    Path(client_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Client>, StatusCode> {
    let storage_guard = storage.read().await;

    let client = storage_guard.get_client(&client_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    info!(
        service = "admin-service",
        event = "client_retrieved",
        client_id = %client_id,
        requested_by = %claims.sub
    );

    Ok(Json(client.clone()))
}

pub async fn create(
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement client creation
    info!(
        service = "admin-service",
        event = "client_create_requested",
        created_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Client creation not yet implemented"
    })))
}

pub async fn update(
    Path(client_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement client update
    info!(
        service = "admin-service",
        event = "client_update_requested",
        client_id = %client_id,
        updated_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Client update not yet implemented"
    })))
}

pub async fn delete(
    Path(client_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement client deletion
    info!(
        service = "admin-service",
        event = "client_delete_requested",
        client_id = %client_id,
        deleted_by = %claims.sub
    );

    Ok(StatusCode::NO_CONTENT)
}

pub async fn rotate_secret(
    Path(client_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement client secret rotation
    info!(
        service = "admin-service",
        event = "client_secret_rotation_requested",
        client_id = %client_id,
        requested_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Client secret rotation not yet implemented",
        "client_secret": "new-secret-placeholder"
    })))
}