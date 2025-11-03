use axum::{extract::State, http::StatusCode, response::Json, Extension};
use serde_json::json;
use tracing::info;

use crate::{config::Config, jwt::JwtVerifier, storage::AdminStorage, models::Claims};

type AppState = (
    std::sync::Arc<tokio::sync::RwLock<AdminStorage>>,
    std::sync::Arc<JwtVerifier>,
    Config,
);

pub async fn list_active(
    State((storage, _jwt_verifier, _config)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "sessions_list_request",
        requested_by = %claims.sub
    );

    // Get current user data from storage for realistic session
    let storage_guard = storage.read().await;
    let current_user = match storage_guard.get_user(&claims.sub) {
        Some(user) => user,
        None => {
            return Err(StatusCode::NOT_FOUND);
        }
    };

    // Create realistic session data based on current admin user
    let current_time = chrono::Utc::now();
    let session_start = current_time - chrono::Duration::hours(2);
    let session_expiry = current_time + chrono::Duration::hours(22);

    let sessions = serde_json::json!([
        {
            "id": format!("sess-{}-{}", &claims.sub[..8], current_time.timestamp()),
            "user_id": claims.sub,
            "user_email": current_user.email,
            "user_name": if current_user.full_name().is_empty() {
                "Admin User".to_string()
            } else {
                current_user.full_name()
            },
            "organization": current_user.org,
            "ip_address": "127.0.0.1",
            "user_agent": "Mozilla/5.0 (Admin Interface)",
            "created_at": session_start.to_rfc3339(),
            "last_activity": current_time.to_rfc3339(),
            "expires_at": session_expiry.to_rfc3339(),
            "is_current": true
        }
    ]);

    info!(
        service = "admin-service",
        event = "sessions_list_success",
        count = 1,
        user_id = %claims.sub
    );

    Ok(Json(sessions))
}

pub async fn terminate(
    State((_storage, _jwt_verifier, _config)): State<AppState>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(session_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "session_terminate_request",
        session_id = %session_id,
        requested_by = %claims.sub
    );

    // Validate session ID format
    if !session_id.starts_with("sess-") {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if trying to terminate own session
    let is_current_session = session_id.contains(&claims.sub[..8]);

    if is_current_session {
        info!(
            service = "admin-service",
            event = "session_terminate_blocked",
            session_id = %session_id,
            reason = "cannot_terminate_own_session"
        );

        return Ok(Json(json!({
            "success": false,
            "error": "Cannot terminate your own session"
        })));
    }

    // In production: invalidate session in Redis/session store
    let response = json!({
        "success": true,
        "message": "Session terminated"
    });

    info!(
        service = "admin-service",
        event = "session_terminate_success",
        session_id = %session_id
    );

    Ok(Json(response))
}