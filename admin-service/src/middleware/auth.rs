use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::warn;

use crate::{config::Config, jwt::JwtVerifier, models::Claims};

type AuthState = (Arc<JwtVerifier>, Config);

pub async fn require_admin(
    State((jwt_verifier, _config)): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let token = extract_bearer_token(&req)
        .ok_or_else(|| {
            warn!(
                service = "admin-service",
                event = "auth_failed",
                reason = "missing_token"
            );
            StatusCode::UNAUTHORIZED
        })?;

    // Verify JWT signature
    let claims = jwt_verifier.verify_token(&token)
        .map_err(|e| {
            warn!(
                service = "admin-service",
                event = "auth_failed",
                reason = "invalid_token",
                error = %e
            );
            StatusCode::UNAUTHORIZED
        })?;

    // Check admin role
    if !jwt_verifier.has_admin_role(&claims) {
        warn!(
            service = "admin-service",
            event = "auth_failed",
            reason = "insufficient_privileges",
            user_id = %claims.sub,
            roles = ?claims.roles
        );
        return Err(StatusCode::FORBIDDEN);
    }

    // Add claims to request extensions for handlers to use
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

fn extract_bearer_token(req: &Request) -> Option<String> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)?
        .to_str()
        .ok()?;

    if auth_header.starts_with("Bearer ") {
        Some(auth_header.trim_start_matches("Bearer ").to_string())
    } else {
        None
    }
}