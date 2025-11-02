use axum::{
    extract::{Query, State},
    response::{Redirect, Response, IntoResponse, Json},
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{config::Config, models::Claims};

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: Option<String>,
}

pub async fn login_redirect(
    Query(params): Query<LoginQuery>,
    State(config): State<Config>
) -> Response {
    let redirect_url = params.redirect.unwrap_or_else(|| "/dashboard".to_string());
    let current_url = format!("{}{}", config.instance.base_url, redirect_url);
    let login_url = format!("{}/?redirect={}",
        config.instance.auth_service_url,
        urlencoding::encode(&current_url));

    Redirect::temporary(&login_url).into_response()
}

#[derive(Serialize)]
pub struct UserInfo {
    id: String,
    email: String,
    first_name: String,
    last_name: String,
    full_name: String,
    admin: Vec<String>,
    org: String,
    roles: Vec<String>,
    created_at: String,
    updated_at: String,
}

pub async fn me(Extension(claims): Extension<Claims>) -> Json<serde_json::Value> {
    let user_info = UserInfo {
        id: claims.sub.clone(),
        email: claims.email.clone(),
        first_name: claims.name.clone(), // Use name field as first name for now
        last_name: "".to_string(),       // No separate last name in claims
        full_name: claims.name.clone(),
        admin: claims.admin.clone(),
        org: claims.org.clone(),
        roles: vec![], // No roles field in current claims
        created_at: claims.iat.to_string(),
        updated_at: claims.iat.to_string(),
    };

    Json(json!(user_info))
}