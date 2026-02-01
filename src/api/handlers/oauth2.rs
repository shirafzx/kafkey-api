use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::application::use_cases::oauth2::OAuth2UseCases;
use crate::domain::entities::user::UserEntity;

/// Response for OAuth2 login success
#[derive(Debug, Serialize)]
pub struct OAuth2LoginResponse {
    pub access_token: String,
    pub user: UserEntity,
}

/// Request to initiate Google OAuth2 flow
pub async fn google_login(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
) -> impl IntoResponse {
    let (auth_url, state, pkce_verifier) = oauth2_use_cases.get_google_auth_url();

    // In a real app, you'd store state and pkce_verifier in session/redis
    // For now, we'll return them to the client to pass back
    Json(serde_json::json!({
        "auth_url": auth_url,
        "state": state,
        "pkce_verifier": pkce_verifier,
    }))
}

/// Query parameters for Google OAuth2 callback
#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
    pub state: String,
    // In production, get these from session/redis
    pub expected_state: String,
    pub pkce_verifier: String,
}

/// Handle Google OAuth2 callback
pub async fn google_callback(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
    Query(params): Query<GoogleCallbackQuery>,
) -> Result<Json<OAuth2LoginResponse>, (StatusCode, String)> {
    let (access_token, user) = oauth2_use_cases
        .handle_google_callback(
            params.code,
            params.state,
            params.expected_state,
            params.pkce_verifier,
        )
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    Ok(Json(OAuth2LoginResponse { access_token, user }))
}

/// Request to initiate GitHub OAuth2 flow
pub async fn github_login(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
) -> impl IntoResponse {
    let (auth_url, state) = oauth2_use_cases.get_github_auth_url();

    // In a real app, you'd store state in session/redis
    Json(serde_json::json!({
        "auth_url": auth_url,
        "state": state,
    }))
}

/// Query parameters for GitHub OAuth2 callback
#[derive(Debug, Deserialize)]
pub struct GitHubCallbackQuery {
    pub code: String,
    pub state: String,
    // In production, get this from session/redis
    pub expected_state: String,
}

/// Handle GitHub OAuth2 callback
pub async fn github_callback(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
    Query(params): Query<GitHubCallbackQuery>,
) -> Result<Json<OAuth2LoginResponse>, (StatusCode, String)> {
    let (access_token, user) = oauth2_use_cases
        .handle_github_callback(params.code, params.state, params.expected_state)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    Ok(Json(OAuth2LoginResponse { access_token, user }))
}
