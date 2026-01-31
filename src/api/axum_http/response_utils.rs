use crate::api::axum_http::middleware::request_id::REQUEST_ID;
use crate::application::dtos::{ApiErrorDetail, ApiErrorResponse, ApiMeta, ApiResponse};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;

pub fn success_response<T: serde::Serialize>(code: &str, message: &str, data: T) -> Response {
    let request_id = REQUEST_ID
        .try_with(|id| id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    let response = ApiResponse {
        success: true,
        code: code.to_string(),
        message: message.to_string(),
        data: Some(data),
        meta: ApiMeta {
            request_id,
            timestamp: Utc::now().to_rfc3339(),
            version: "1.0".to_string(),
        },
    };

    (StatusCode::OK, Json(response)).into_response()
}

pub fn error_response(
    status: StatusCode,
    code: &str,
    message: &str,
    errors: Option<Vec<ApiErrorDetail>>,
) -> Response {
    let request_id = REQUEST_ID
        .try_with(|id| id.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    let response = ApiErrorResponse {
        success: false,
        code: code.to_string(),
        message: message.to_string(),
        errors,
        meta: ApiMeta {
            request_id,
            timestamp: Utc::now().to_rfc3339(),
            version: "1.0".to_string(),
        },
    };

    (status, Json(response)).into_response()
}
