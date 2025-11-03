use axum::{extract::State, http::StatusCode, response::Json, Extension};
use serde_json::{json, Value};
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{config::Config, jwt::JwtVerifier, storage::AdminStorage, models::Claims};

type AppState = (Arc<RwLock<AdminStorage>>, Arc<JwtVerifier>, Config);

pub async fn users_stats(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "users_stats_request",
        requested_by = %claims.sub
    );

    let storage_guard = storage.read().await;
    let users_count = storage_guard.users_count();

    // Mock data for demo - in real implementation, calculate from actual data
    let response = json!({
        "total_users": users_count,
        "growth_percentage": 12.5,
        "active_users": users_count - 1,
        "new_users_today": 2,
        "new_users_week": 8
    });

    Ok(Json(response))
}

pub async fn sessions_stats(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "sessions_stats_request",
        requested_by = %claims.sub
    );

    // Mock data for demo
    let response = json!({
        "active_sessions": 15,
        "change_percentage": 8.3,
        "total_sessions_today": 42,
        "unique_users_today": 28
    });

    Ok(Json(response))
}

pub async fn organizations_stats(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "organizations_stats_request",
        requested_by = %claims.sub
    );

    let storage_guard = storage.read().await;
    let orgs_count = storage_guard.organizations_count();

    let response = json!({
        "total_organizations": orgs_count,
        "growth_percentage": 5.2,
        "active_organizations": orgs_count
    });

    Ok(Json(response))
}

pub async fn clients_stats(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "clients_stats_request",
        requested_by = %claims.sub
    );

    let storage_guard = storage.read().await;
    let clients_count = storage_guard.clients_count();

    let response = json!({
        "total_clients": clients_count,
        "change_percentage": -2.1,
        "active_clients": clients_count
    });

    Ok(Json(response))
}

pub async fn activity_data(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "activity_data_request",
        requested_by = %claims.sub
    );

    // Mock activity data for the last 7 days
    let response = json!([
        {
            "date": "2024-11-01",
            "logins": 45,
            "registrations": 3,
            "active_users": 42
        },
        {
            "date": "2024-11-02",
            "logins": 52,
            "registrations": 5,
            "active_users": 48
        },
        {
            "date": "2024-11-03",
            "logins": 38,
            "registrations": 2,
            "active_users": 35
        }
    ]);

    Ok(Json(response))
}

pub async fn login_distribution(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "login_distribution_request",
        requested_by = %claims.sub
    );

    // Mock login distribution by organization
    let response = json!([
        {
            "org": "default",
            "count": 125,
            "percentage": 45.5
        },
        {
            "org": "group-7a",
            "count": 89,
            "percentage": 32.4
        },
        {
            "org": "group-8b",
            "count": 61,
            "percentage": 22.1
        }
    ]);

    Ok(Json(response))
}

pub async fn recent_activities(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "recent_activities_request",
        requested_by = %claims.sub
    );

    // Mock recent activities
    let response = json!([
        {
            "id": "act-001",
            "type": "login",
            "user_email": "user@example.com",
            "org": "default",
            "timestamp": "2024-11-03T15:30:00Z",
            "details": "Successful login from 192.168.1.100"
        },
        {
            "id": "act-002",
            "type": "registration",
            "user_email": "newuser@example.com",
            "org": "group-7a",
            "timestamp": "2024-11-03T14:45:00Z",
            "details": "New user registration"
        }
    ]);

    Ok(Json(response))
}

pub async fn quick_stats(
    State((storage, _, _)): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        service = "admin-service",
        event = "quick_stats_request",
        requested_by = %claims.sub
    );

    let response = json!({
        "last_login_time": "2024-11-03T15:30:00Z",
        "failed_logins_today": 3,
        "new_users_week": 12
    });

    Ok(Json(response))
}