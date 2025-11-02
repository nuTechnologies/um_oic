use axum::{
    extract::{Request, State},
    http::{header, StatusCode, HeaderMap, HeaderValue},
    middleware::Next,
    response::{Response, Redirect, IntoResponse},
};
use std::sync::Arc;
use tracing::warn;

use crate::{config::Config, jwt::JwtVerifier, models::Claims};

type AuthState = (Arc<JwtVerifier>, Config);

pub async fn require_admin(
    State((jwt_verifier, config)): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Check if there's a token in localStorage or query parameter first
    let token = extract_bearer_token(&req)
        .or_else(|| extract_query_token(&req))
        .or_else(|| extract_cookie_token(&req));

    if token.is_none() {
        // Check if this is a browser request (not an API call)
        let is_browser_request = is_browser_request(&req);
        let path = req.uri().path();

        if is_browser_request && !path.starts_with("/api/") {
            // Redirect to auth service for browser requests
            let current_url = format!("{}{}", config.instance.base_url, req.uri());
            let login_url = format!("{}/?redirect={}",
                config.instance.auth_service_url,
                urlencoding::encode(&current_url));

            warn!(
                service = "admin-service",
                event = "auth_redirect",
                reason = "missing_token",
                redirect_to = %login_url
            );

            return Ok(Redirect::temporary(&login_url).into_response());
        } else {
            // Return 401 for API requests
            warn!(
                service = "admin-service",
                event = "auth_failed",
                reason = "missing_token"
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    let token = token.unwrap();

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
            admin = ?claims.admin
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

fn extract_query_token(req: &Request) -> Option<String> {
    let query = req.uri().query()?;
    for param in query.split('&') {
        if let Some((key, value)) = param.split_once('=') {
            if key == "token" {
                return Some(urlencoding::decode(value).ok()?.into_owned());
            }
        }
    }
    None
}

fn extract_cookie_token(req: &Request) -> Option<String> {
    let cookie_header = req.headers()
        .get(header::COOKIE)?
        .to_str()
        .ok()?;

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some((key, value)) = cookie.split_once('=') {
            if key == "auth_token" {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn is_browser_request(req: &Request) -> bool {
    if let Some(accept) = req.headers().get(header::ACCEPT) {
        if let Ok(accept_str) = accept.to_str() {
            return accept_str.contains("text/html");
        }
    }
    false
}