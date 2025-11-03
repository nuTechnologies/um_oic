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
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<crate::models::CreateClientRequest>,
) -> Result<Json<Client>, StatusCode> {
    info!(
        service = "admin-service",
        event = "client_create_requested",
        client_id = %request.client_id,
        created_by = %claims.sub
    );

    // Generate client secret for confidential clients
    let client_secret_hash = if request.client_type == crate::models::ClientType::Confidential {
        let secret = format!("cs_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
        Some(crate::password::hash_password(&secret).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
    } else {
        None
    };

    let client = Client {
        client_id: request.client_id.clone(),
        client_secret_hash,
        name: request.name,
        client_type: request.client_type,
        redirect_uris: request.redirect_uris,
        allowed_scopes: request.allowed_scopes,
        require_pkce: request.require_pkce.unwrap_or(false),
        grant_types: request.grant_types.unwrap_or_else(|| vec!["authorization_code".to_string()]),
        created_at: time::OffsetDateTime::now_utc(),
    };

    let mut storage_guard = storage.write().await;

    // Check if client already exists
    if storage_guard.get_client(&request.client_id).is_some() {
        return Err(StatusCode::CONFLICT);
    }

    match storage_guard.add_client(client.clone()).await {
        Ok(created_client) => {
            info!(
                service = "admin-service",
                event = "client_created",
                client_id = %request.client_id,
                created_by = %claims.sub
            );
            Ok(Json(created_client))
        }
        Err(e) => {
            tracing::error!("Failed to create client: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    Path(client_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<crate::models::UpdateClientRequest>,
) -> Result<Json<Client>, StatusCode> {
    info!(
        service = "admin-service",
        event = "client_update_requested",
        client_id = %client_id,
        updated_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Get existing client
    let existing_client = storage_guard.get_client(&client_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    // Update client with provided fields
    let updated_client = Client {
        client_id: existing_client.client_id.clone(),
        client_secret_hash: existing_client.client_secret_hash,
        name: request.name.unwrap_or(existing_client.name),
        client_type: existing_client.client_type,
        redirect_uris: request.redirect_uris.unwrap_or(existing_client.redirect_uris),
        allowed_scopes: request.allowed_scopes.unwrap_or(existing_client.allowed_scopes),
        require_pkce: request.require_pkce.unwrap_or(existing_client.require_pkce),
        grant_types: request.grant_types.unwrap_or(existing_client.grant_types),
        created_at: existing_client.created_at,
    };

    match storage_guard.update_client(&client_id, updated_client.clone()).await {
        Ok(client) => {
            info!(
                service = "admin-service",
                event = "client_updated",
                client_id = %client_id,
                updated_by = %claims.sub
            );
            Ok(Json(client))
        }
        Err(e) => {
            tracing::error!("Failed to update client: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    Path(client_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    info!(
        service = "admin-service",
        event = "client_delete_requested",
        client_id = %client_id,
        deleted_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Check if client exists
    if storage_guard.get_client(&client_id).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    match storage_guard.delete_client(&client_id).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "client_deleted",
                client_id = %client_id,
                deleted_by = %claims.sub
            );
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            tracing::error!("Failed to delete client: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn rotate_secret(
    Path(client_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "client_secret_rotation_requested",
        client_id = %client_id,
        requested_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Get existing client
    let existing_client = storage_guard.get_client(&client_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    // Only confidential clients have secrets
    if existing_client.client_type != crate::models::ClientType::Confidential {
        return Ok(Json(json!({
            "success": false,
            "error": "Only confidential clients have secrets to rotate"
        })));
    }

    // Generate new secret
    let new_secret = format!("cs_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let new_secret_hash = crate::password::hash_password(&new_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update client with new secret hash
    let updated_client = Client {
        client_secret_hash: Some(new_secret_hash),
        ..existing_client
    };

    match storage_guard.update_client(&client_id, updated_client).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "client_secret_rotated",
                client_id = %client_id,
                requested_by = %claims.sub
            );

            Ok(Json(json!({
                "success": true,
                "client_secret": new_secret,
                "message": "Client secret has been rotated. Save the new secret securely."
            })))
        }
        Err(e) => {
            tracing::error!("Failed to rotate client secret: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}