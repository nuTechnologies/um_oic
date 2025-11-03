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
    models::{Claims, UserResponse, Organization, CreateOrganizationRequest, UpdateOrganizationRequest},
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn list(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Organization>>, StatusCode> {
    let storage_guard = storage.read().await;

    let organizations: Vec<Organization> = storage_guard.get_all_organizations()
        .cloned()
        .collect();

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
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<Json<Organization>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_create_request",
        org_id = %request.id,
        created_by = %claims.sub
    );

    let organization = Organization {
        id: request.id.clone(),
        name: request.name,
        description: request.description,
        metadata: request.metadata.unwrap_or_default(),
        created_at: time::OffsetDateTime::now_utc(),
    };

    let mut storage_guard = storage.write().await;

    // Check if organization already exists
    if storage_guard.get_organization(&request.id).is_some() {
        return Err(StatusCode::CONFLICT);
    }

    match storage_guard.add_organization(organization.clone()).await {
        Ok(created_org) => {
            info!(
                service = "admin-service",
                event = "organization_created",
                org_id = %request.id,
                created_by = %claims.sub
            );
            Ok(Json(created_org))
        }
        Err(e) => {
            tracing::error!("Failed to create organization: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update(
    Path(org_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateOrganizationRequest>,
) -> Result<Json<Organization>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_update_request",
        org_id = %org_id,
        updated_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Get existing organization
    let existing_org = storage_guard.get_organization(&org_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    // Update organization with provided fields
    let updated_org = Organization {
        id: existing_org.id.clone(),
        name: request.name.unwrap_or(existing_org.name),
        description: request.description.unwrap_or(existing_org.description),
        metadata: request.metadata.unwrap_or(existing_org.metadata),
        created_at: existing_org.created_at,
    };

    match storage_guard.update_organization(&org_id, updated_org.clone()).await {
        Ok(org) => {
            info!(
                service = "admin-service",
                event = "organization_updated",
                org_id = %org_id,
                updated_by = %claims.sub
            );
            Ok(Json(org))
        }
        Err(e) => {
            tracing::error!("Failed to update organization: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete(
    Path(org_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    info!(
        service = "admin-service",
        event = "organization_delete_request",
        org_id = %org_id,
        deleted_by = %claims.sub
    );

    let mut storage_guard = storage.write().await;

    // Check if organization exists
    if storage_guard.get_organization(&org_id).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    match storage_guard.delete_organization(&org_id).await {
        Ok(_) => {
            info!(
                service = "admin-service",
                event = "organization_deleted",
                org_id = %org_id,
                deleted_by = %claims.sub
            );
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            tracing::error!("Failed to delete organization: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}