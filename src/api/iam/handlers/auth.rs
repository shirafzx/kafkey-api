use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::application::use_cases::iam::auth_use_cases::{
    AuthUseCases, LoginRequest, RefreshTokenRequest, RegisterRequest,
};

pub async fn login<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<Value>, StatusCode> {
    match auth_use_cases.login(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Login error: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn register<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<Value>, StatusCode> {
    match auth_use_cases.register(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Registration error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn refresh_token<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<Value>, StatusCode> {
    match auth_use_cases.refresh_token(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Token refresh error: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn logout<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    Json(token): Json<Value>,
) -> Result<StatusCode, StatusCode> {
    let token = token
        .get("token")
        .and_then(|t| t.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match auth_use_cases.logout(token.to_string()).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            tracing::error!("Logout error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn validate_token<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    Json(token): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let token = token
        .get("token")
        .and_then(|t| t.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match auth_use_cases.validate_token(token.to_string()).await {
        Ok(Some(user)) => Ok(Json(json!(user))),
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            tracing::error!("Token validation error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
