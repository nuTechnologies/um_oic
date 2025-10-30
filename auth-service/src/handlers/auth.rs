use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::{
    config::Config,
    jwt::JwtService,
    models::{LoginRequest, LoginResponse, UserStatus},
    password,
    storage::FileStorage,
};

type AppState = (Arc<RwLock<FileStorage>>, Arc<JwtService>, Config);

pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State((storage, jwt_service, config)): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let storage_guard = storage.read().await;

    // Find user by email
    let user = match storage_guard.get_user_by_email(&request.email) {
        Some(user) => user,
        None => {
            warn!(
                service = "auth-service",
                event = "login",
                email = %request.email,
                success = false,
                reason = "user_not_found",
                ip = %addr.ip()
            );
            return Ok(Json(LoginResponse {
                success: false,
                access_token: None,
                refresh_token: None,
                expires_in: None,
                requires_mfa: false,
                mfa_session: None,
                redirect_to: None,
            }));
        }
    };

    // Check if user is active
    if !user.is_active() {
        warn!(
            service = "auth-service",
            event = "login",
            email = %request.email,
            success = false,
            reason = "user_inactive",
            ip = %addr.ip()
        );
        return Ok(Json(LoginResponse {
            success: false,
            access_token: None,
            refresh_token: None,
            expires_in: None,
            requires_mfa: false,
            mfa_session: None,
            redirect_to: None,
        }));
    }

    // Verify password
    let password_valid = match password::verify_password(&request.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => {
            warn!(
                service = "auth-service",
                event = "password_verification_error",
                error = %e,
                ip = %addr.ip()
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !password_valid {
        warn!(
            service = "auth-service",
            event = "login",
            email = %request.email,
            success = false,
            reason = "invalid_password",
            ip = %addr.ip()
        );
        return Ok(Json(LoginResponse {
            success: false,
            access_token: None,
            refresh_token: None,
            expires_in: None,
            requires_mfa: false,
            mfa_session: None,
            redirect_to: None,
        }));
    }

    // Check if MFA is required
    if config.security.require_mfa && user.mfa_secret.is_some() {
        // TODO: Implement MFA session handling
        warn!(
            service = "auth-service",
            event = "mfa_required",
            user_id = %user.id,
            ip = %addr.ip()
        );
        return Ok(Json(LoginResponse {
            success: true,
            access_token: None,
            refresh_token: None,
            expires_in: None,
            requires_mfa: true,
            mfa_session: Some("mfa-session-placeholder".to_string()),
            redirect_to: None,
        }));
    }

    // Create JWT tokens
    let access_token = match jwt_service.create_token(
        user,
        vec!["auth-service".to_string()],
        &config.instance.issuer,
        config.security.access_token_ttl,
    ) {
        Ok(token) => token,
        Err(e) => {
            warn!(
                service = "auth-service",
                event = "jwt_creation_failed",
                error = %e,
                user_id = %user.id
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let refresh_token = match jwt_service.create_token(
        user,
        vec!["auth-service".to_string()],
        &config.instance.issuer,
        config.security.refresh_token_ttl,
    ) {
        Ok(token) => token,
        Err(e) => {
            warn!(
                service = "auth-service",
                event = "refresh_token_creation_failed",
                error = %e,
                user_id = %user.id
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!(
        service = "auth-service",
        event = "login",
        email = %request.email,
        user_id = %user.id,
        success = true,
        ip = %addr.ip()
    );

    Ok(Json(LoginResponse {
        success: true,
        access_token: Some(access_token),
        refresh_token: Some(refresh_token),
        expires_in: Some(config.security.access_token_ttl),
        requires_mfa: false,
        mfa_session: None,
        redirect_to: Some("/dashboard".to_string()),
    }))
}

pub async fn logout(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(_): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // In a stateless JWT system, logout is primarily client-side
    // We just log the event and return success

    info!(
        service = "auth-service",
        event = "logout",
        ip = %addr.ip()
    );

    Ok(Json(json!({
        "success": true,
        "message": "Logged out successfully"
    })))
}

pub async fn forgot_password(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(_): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement password reset functionality
    info!(
        service = "auth-service",
        event = "forgot_password",
        ip = %addr.ip()
    );

    Ok(Json(json!({
        "success": true,
        "message": "If the email exists, a reset link has been sent"
    })))
}

pub async fn reset_password(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(_): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement password reset functionality
    info!(
        service = "auth-service",
        event = "reset_password",
        ip = %addr.ip()
    );

    Ok(Json(json!({
        "success": true,
        "message": "Password reset successfully"
    })))
}