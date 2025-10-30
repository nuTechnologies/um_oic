use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    config::Config,
    jwt::JwtService,
    models::{OAuth2AuthorizeRequest, OAuth2TokenRequest, OAuth2TokenResponse, UserInfo},
    storage::FileStorage,
};

type AppState = (Arc<RwLock<FileStorage>>, Arc<JwtService>, Config);

pub async fn authorize(
    Query(params): Query<OAuth2AuthorizeRequest>,
    State((storage, _, config)): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    // TODO: Implement full OAuth2 authorization flow
    // For now, return a placeholder response

    tracing::info!(
        service = "auth-service",
        event = "oauth2_authorize",
        client_id = %params.client_id,
        response_type = %params.response_type
    );

    Ok(Json(json!({
        "status": "redirect",
        "location": "/login.html",
        "client_id": params.client_id,
        "redirect_uri": params.redirect_uri,
        "state": params.state
    })))
}

pub async fn token(
    State((storage, jwt_service, config)): State<AppState>,
    Json(request): Json<OAuth2TokenRequest>,
) -> Result<Json<OAuth2TokenResponse>, StatusCode> {
    // TODO: Implement full OAuth2 token exchange
    // For now, return a placeholder response

    tracing::info!(
        service = "auth-service",
        event = "oauth2_token",
        grant_type = %request.grant_type,
        client_id = %request.client_id
    );

    // Placeholder token response
    Ok(Json(OAuth2TokenResponse {
        access_token: "placeholder-access-token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: config.security.access_token_ttl,
        refresh_token: Some("placeholder-refresh-token".to_string()),
        scope: "openid profile email".to_string(),
    }))
}

pub async fn userinfo(
    State((storage, jwt_service, config)): State<AppState>,
) -> Result<Json<UserInfo>, StatusCode> {
    // TODO: Extract and validate Bearer token from Authorization header
    // For now, return a placeholder response

    tracing::info!(
        service = "auth-service",
        event = "oauth2_userinfo"
    );

    Ok(Json(UserInfo {
        sub: "placeholder-user-id".to_string(),
        email: "placeholder@example.com".to_string(),
        name: "Placeholder User".to_string(),
        given_name: "Placeholder".to_string(),
        family_name: "User".to_string(),
        roles: vec!["user".to_string()],
        permissions: vec!["read".to_string()],
        custom_claims: std::collections::HashMap::new(),
    }))
}

pub async fn discovery(
    State((_, _, config)): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "issuer": config.instance.issuer,
        "authorization_endpoint": format!("{}/oauth2/authorize", config.instance.issuer),
        "token_endpoint": format!("{}/oauth2/token", config.instance.issuer),
        "userinfo_endpoint": format!("{}/oauth2/userinfo", config.instance.issuer),
        "jwks_uri": format!("{}/oauth2/jwks", config.instance.issuer),
        "scopes_supported": [
            "openid",
            "profile",
            "email"
        ],
        "response_types_supported": [
            "code"
        ],
        "grant_types_supported": [
            "authorization_code",
            "refresh_token"
        ],
        "subject_types_supported": [
            "public"
        ],
        "id_token_signing_alg_values_supported": [
            "HS256"
        ],
        "token_endpoint_auth_methods_supported": [
            "client_secret_post",
            "client_secret_basic"
        ],
        "code_challenge_methods_supported": [
            "S256"
        ]
    })))
}