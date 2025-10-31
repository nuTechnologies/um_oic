use axum::{
    extract::Query,
    response::{Redirect, Response, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginQuery {
    redirect: Option<String>,
}

pub async fn login_redirect(Query(params): Query<LoginQuery>) -> Response {
    let redirect_url = params.redirect.unwrap_or_else(|| "/dashboard".to_string());
    let current_url = format!("http://localhost:8444{}", redirect_url);
    let login_url = format!("https://localhost:8443/?redirect={}",
        urlencoding::encode(&current_url));

    Redirect::temporary(&login_url).into_response()
}