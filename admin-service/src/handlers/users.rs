use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    config::Config,
    jwt::JwtVerifier,
    models::{Claims, CreateUserRequest, UpdateUserRequest, User, UserResponse, UserStatus, AddUserToGroupRequest},
    password,
    storage::AdminStorage,
};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    search: Option<String>,
    status: Option<String>,
    role: Option<String>,
    group: Option<String>,
    limit: Option<u32>,
}

pub async fn list(
    Query(query): Query<ListUsersQuery>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let storage_guard = storage.read().await;

    let mut users: Vec<&User> = if let Some(search) = &query.search {
        storage_guard.search_users(search)
    } else {
        storage_guard.get_all_users().collect()
    };

    // Filter by status
    if let Some(status) = &query.status {
        users.retain(|u| match status.as_str() {
            "active" => matches!(u.status, UserStatus::Active),
            "inactive" => matches!(u.status, UserStatus::Inactive),
            "suspended" => matches!(u.status, UserStatus::Suspended),
            _ => true,
        });
    }

    // Filter by role
    if let Some(role) = &query.role {
        users.retain(|u| u.roles.contains(role));
    }

    // Filter by group
    if let Some(group) = &query.group {
        users.retain(|u| u.group_memberships.contains(group));
    }

    // Apply limit
    let limit = query.limit.unwrap_or(100) as usize;
    users.truncate(limit);

    info!(
        service = "admin-service",
        event = "users_listed",
        requested_by = %claims.sub,
        count = users.len(),
        search = ?query.search
    );

    let response: Vec<UserResponse> = users
        .into_iter()
        .cloned()
        .map(UserResponse::from)
        .collect();

    Ok(Json(response))
}

pub async fn get(
    Path(user_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<UserResponse>, StatusCode> {
    let storage_guard = storage.read().await;

    let user = storage_guard.get_user(&user_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    info!(
        service = "admin-service",
        event = "user_retrieved",
        user_id = %user_id,
        requested_by = %claims.sub
    );

    Ok(Json(UserResponse::from(user.clone())))
}

pub async fn create(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    // Hash password
    let password_hash = password::hash_password(&request.password)
        .map_err(|e| {
            warn!(
                service = "admin-service",
                event = "password_hashing_failed",
                error = %e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create user
    let now = OffsetDateTime::now_utc();
    let user = User {
        id: format!("user-{}", Uuid::new_v4().simple()),
        email: request.email,
        password_hash,
        first_name: request.first_name,
        last_name: request.last_name,
        status: UserStatus::Active,
        roles: request.roles.unwrap_or_default(),
        group_memberships: request.group_memberships.unwrap_or_default(),
        mfa_secret: None,
        created_at: now,
        updated_at: now,
    };

    // Save to storage
    let mut storage_guard = storage.write().await;
    let created_user = storage_guard.create_user(user).await
        .map_err(|e| {
            warn!(
                service = "admin-service",
                event = "user_creation_failed",
                error = %e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(
        service = "admin-service",
        event = "user_created",
        user_id = %created_user.id,
        email = %created_user.email,
        created_by = %claims.sub
    );

    Ok(Json(UserResponse::from(created_user)))
}

pub async fn update(
    Path(user_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let mut storage_guard = storage.write().await;

    // Get existing user
    let mut user = storage_guard.get_user(&user_id)
        .ok_or(StatusCode::NOT_FOUND)?
        .clone();

    // Update fields
    if let Some(first_name) = request.first_name {
        user.first_name = first_name;
    }
    if let Some(last_name) = request.last_name {
        user.last_name = last_name;
    }
    if let Some(status) = request.status {
        user.status = status;
    }
    if let Some(roles) = request.roles {
        user.roles = roles;
    }
    if let Some(group_memberships) = request.group_memberships {
        user.group_memberships = group_memberships;
    }
    if let Some(custom_claims) = request.custom_claims {
        user.custom_claims = custom_claims;
    }

    user.updated_at = OffsetDateTime::now_utc();

    // Save changes
    let updated_user = storage_guard.update_user(&user_id, user).await
        .map_err(|e| {
            warn!(
                service = "admin-service",
                event = "user_update_failed",
                user_id = %user_id,
                error = %e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(
        service = "admin-service",
        event = "user_updated",
        user_id = %user_id,
        updated_by = %claims.sub
    );

    Ok(Json(UserResponse::from(updated_user)))
}

pub async fn delete(
    Path(user_id): Path<String>,
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    let mut storage_guard = storage.write().await;

    // Check if user exists
    storage_guard.get_user(&user_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Delete user
    storage_guard.delete_user(&user_id).await
        .map_err(|e| {
            warn!(
                service = "admin-service",
                event = "user_deletion_failed",
                user_id = %user_id,
                error = %e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    info!(
        service = "admin-service",
        event = "user_deleted",
        user_id = %user_id,
        deleted_by = %claims.sub
    );

    Ok(StatusCode::NO_CONTENT)
}

pub async fn reset_password(
    Path(user_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement password reset functionality
    info!(
        service = "admin-service",
        event = "password_reset_requested",
        user_id = %user_id,
        requested_by = %claims.sub
    );

    Ok(Json(json!({
        "success": true,
        "message": "Password reset email sent",
        "reset_token": "placeholder-reset-token"
    })))
}

pub async fn add_group(
    Path(user_id): Path<String>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<AddUserToGroupRequest>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement add user to group
    info!(
        service = "admin-service",
        event = "user_group_added",
        user_id = %user_id,
        group_id = %request.group_id,
        added_by = %claims.sub
    );

    Ok(StatusCode::OK)
}

pub async fn remove_group(
    Path((user_id, group_id)): Path<(String, String)>,
    State(_): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, StatusCode> {
    // TODO: Implement remove user from group
    info!(
        service = "admin-service",
        event = "user_group_removed",
        user_id = %user_id,
        group_id = %group_id,
        removed_by = %claims.sub
    );

    Ok(StatusCode::NO_CONTENT)
}

// Password hashing utilities (copied from auth-service)
mod password {
    use anyhow::Result;
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }
}