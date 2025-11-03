use axum::{extract::{State, Path}, http::StatusCode, response::Json, Extension};
use anyhow::Result;
use serde_json::{json, Value};
use tracing::{info, warn};

use crate::{
    config::Config,
    jwt::JwtVerifier,
    storage::AdminStorage,
    models::{Claims, ClaimDefinition, CreateClaimRequest, UpdateClaimRequest, UpdateClaimsRegistryRequest}
};

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
        "version": "1.0",
        "claims": claims_registry.claims,
        "last_updated": "2024-10-30T12:00:00Z"
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
    Json(request): Json<CreateClaimRequest>,
) -> Result<Json<ClaimDefinition>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_create_request",
        claim_key = %request.key,
        created_by = %claims.sub
    );

    let claim_definition = ClaimDefinition {
        claim_type: request.claim_type,
        items: request.items,
        description: request.description,
        default_allowed: request.default_allowed,
        required: request.required,
        sensitive: request.sensitive,
        admin_only: request.admin_only,
    };

    let mut storage_guard = storage.write().await;

    // Check if claim already exists
    if storage_guard.get_claims().claims.contains_key(&request.key) {
        return Err(StatusCode::CONFLICT);
    }

    match storage_guard.add_claim(request.key.clone(), claim_definition.clone()).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "claim_created",
                claim_key = %request.key,
                created_by = %claims.sub
            );
            Ok(Json(claim_definition))
        }
        Err(e) => {
            tracing::error!("Failed to create claim: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    Path(claim_key): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateClaimRequest>,
) -> Result<Json<ClaimDefinition>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_update_request",
        claim_key = %claim_key,
        updated_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Get existing claim
    let existing_claim = storage_guard.get_claims().claims.get(&claim_key)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    // Update claim with provided fields
    let updated_claim = ClaimDefinition {
        claim_type: request.claim_type.unwrap_or(existing_claim.claim_type),
        items: request.items.or(existing_claim.items),
        description: request.description.unwrap_or(existing_claim.description),
        default_allowed: request.default_allowed.unwrap_or(existing_claim.default_allowed),
        required: request.required.or(existing_claim.required),
        sensitive: request.sensitive.or(existing_claim.sensitive),
        admin_only: request.admin_only.or(existing_claim.admin_only),
    };

    match storage_guard.update_claim(&claim_key, updated_claim.clone()).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "claim_updated",
                claim_key = %claim_key,
                updated_by = %claims.sub
            );
            Ok(Json(updated_claim))
        }
        Err(e) => {
            tracing::error!("Failed to update claim: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_registry(
    State((storage, _, _)): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claims_registry_request"
    );

    let claims_registry = {
        let storage_guard = storage.read().await;
        storage_guard.get_claims().clone()
    };

    let response = serde_json::json!({
        "version": "1.0",
        "claims": claims_registry.claims,
        "last_updated": "2024-10-30T12:00:00Z"
    });

    Ok(Json(response))
}

pub async fn update_registry(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateClaimsRegistryRequest>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "claims_registry_update_request",
        requested_by = %claims.sub,
        count = request.claims.len()
    );

    let new_registry = crate::models::ClaimsRegistry {
        claims: request.claims,
    };

    let mut storage_guard = storage.write().await;

    match storage_guard.update_claims_registry(new_registry).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "claims_registry_updated",
                requested_by = %claims.sub
            );
            Ok(Json(json!({
                "success": true,
                "message": "Claims registry updated successfully"
            })))
        }
        Err(e) => {
            tracing::error!("Failed to update claims registry: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    Path(claim_key): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    info!(
        service = "admin-service",
        event = "claim_delete_request",
        claim_key = %claim_key,
        deleted_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Check if claim exists
    if !storage_guard.get_claims().claims.contains_key(&claim_key) {
        return Err(StatusCode::NOT_FOUND);
    }

    match storage_guard.delete_claim(&claim_key).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "claim_deleted",
                claim_key = %claim_key,
                deleted_by = %claims.sub
            );
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            tracing::error!("Failed to delete claim: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}