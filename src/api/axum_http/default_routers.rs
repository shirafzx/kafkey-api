use crate::api::axum_http::response_utils::{error_response, success_response};
use axum::{http::StatusCode, response::IntoResponse};

pub async fn not_found() -> impl IntoResponse {
    error_response(
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "The requested resource was not found",
        None,
    )
}

pub async fn health_check() -> impl IntoResponse {
    success_response(
        "HEALTH_OK",
        "Service is healthy",
        serde_json::json!({ "status": "UP" }),
    )
}
