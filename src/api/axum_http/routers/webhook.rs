use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::axum_http::response_utils::{error_response, success_response},
    application::{
        dtos::webhook::{CreateWebhookRequest, UpdateWebhookRequest},
        use_cases::webhook::WebhookUseCases,
    },
    services::jwt_service::TokenClaims,
};

#[derive(serde::Deserialize)]
struct PaginationParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn webhook_router(webhook_use_cases: Arc<WebhookUseCases>) -> Router {
    Router::new()
        .route("/", post(create_webhook).get(list_webhooks))
        .route(
            "/:id",
            get(get_webhook).put(update_webhook).delete(delete_webhook),
        )
        .route("/:id/deliveries", get(list_deliveries))
        .with_state(webhook_use_cases)
}

async fn create_webhook(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
    Json(payload): Json<CreateWebhookRequest>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    if let Err(e) = payload.validate() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            &e.to_string(),
            None,
        );
    }

    match use_cases.create_webhook(tenant_id, payload).await {
        Ok(response) => success_response("CREATED", "Webhook created successfully", response),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "CREATION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn list_webhooks(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    match use_cases.list_webhooks(tenant_id).await {
        Ok(response) => success_response("OK", "Webhooks retrieved successfully", response),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "RETRIEVAL_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn get_webhook(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    match use_cases.get_webhook(id, tenant_id).await {
        Ok(response) => success_response("OK", "Webhook details", response),
        Err(_) => error_response(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Webhook not found",
            None,
        ),
    }
}

async fn update_webhook(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateWebhookRequest>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    if let Err(e) = payload.validate() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            &e.to_string(),
            None,
        );
    }

    match use_cases.update_webhook(id, tenant_id, payload).await {
        Ok(response) => success_response("OK", "Webhook updated successfully", response),
        Err(_) => error_response(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Webhook not found or update failed",
            None,
        ),
    }
}

async fn delete_webhook(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    match use_cases.delete_webhook(id, tenant_id).await {
        Ok(_) => success_response("OK", "Webhook deleted successfully", ()),
        Err(_) => error_response(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Webhook not found or delete failed",
            None,
        ),
    }
}

async fn list_deliveries(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<WebhookUseCases>>,
    Path(id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&claims.tenant_id.clone().unwrap_or_default()) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_TENANT_ID",
                "Invalid tenant ID",
                None,
            );
        }
    };

    let limit = pagination.limit.unwrap_or(20);
    let offset = pagination.offset.unwrap_or(0);

    match use_cases
        .list_deliveries(id, tenant_id, limit, offset)
        .await
    {
        Ok(response) => {
            success_response("OK", "Webhook deliveries retrieved successfully", response)
        }
        Err(_) => error_response(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            "Webhook not found",
            None,
        ),
    }
}
