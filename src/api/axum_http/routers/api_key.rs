use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::axum_http::response_utils::{error_response, success_response},
    application::{
        dtos::{ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse},
        use_cases::api_key::ApiKeyUseCases,
    },
    domain::entities::api_key::ApiKeyEntity,
};

pub fn api_key_router(api_key_use_cases: Arc<ApiKeyUseCases>) -> Router {
    Router::new()
        .route(
            "/tenants/:tenant_id/keys",
            post(create_api_key).get(list_api_keys),
        )
        .route("/tenants/:tenant_id/keys/:key_id", delete(delete_api_key))
        .route(
            "/tenants/:tenant_id/keys/:key_id/revoke",
            post(revoke_api_key),
        )
        .with_state(api_key_use_cases)
}

/// Create a new API key for a tenant
async fn create_api_key(
    State(use_cases): State<Arc<ApiKeyUseCases>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> impl IntoResponse {
    // Validate
    if let Err(e) = payload.validate() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            &e.to_string(),
            None,
        );
    }

    // TODO: Verify that auth_user owns this tenant
    // For now, we'll trust the JWT authentication

    // Create API key
    let (key_id, plain_key) = match use_cases
        .create(tenant_id, payload.name.clone(), payload.environment.clone())
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "API_KEY_CREATE_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    // Extract prefix from plain key
    let key_prefix = format!(
        "{}...{}",
        &plain_key[..10],
        &plain_key[plain_key.len() - 4..]
    );

    success_response(
        "API_KEY_CREATED",
        "API key created successfully. Save this key securely - it won't be shown again.",
        CreateApiKeyResponse {
            id: key_id.to_string(),
            key: plain_key,
            key_prefix,
            name: payload.name,
            environment: payload.environment,
        },
    )
}

/// List all API keys for a tenant
async fn list_api_keys(
    State(use_cases): State<Arc<ApiKeyUseCases>>,
    Path(tenant_id): Path<Uuid>,
) -> impl IntoResponse {
    // TODO: Verify that auth_user owns this tenant

    let keys = match use_cases.get_by_tenant(tenant_id).await {
        Ok(keys) => keys,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "FETCH_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    let responses: Vec<ApiKeyResponse> = keys.into_iter().map(map_to_response).collect();

    success_response(
        "API_KEYS_FETCHED",
        "API keys retrieved successfully",
        responses,
    )
}

/// Revoke an API key
async fn revoke_api_key(
    State(use_cases): State<Arc<ApiKeyUseCases>>,
    Path((tenant_id, key_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    // TODO: Verify that auth_user owns this tenant

    if let Err(e) = use_cases.revoke(key_id, tenant_id).await {
        return error_response(
            StatusCode::BAD_REQUEST,
            "REVOKE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "API_KEY_REVOKED",
        "API key revoked successfully",
        serde_json::json!({"success": true}),
    )
}

/// Delete an API key
async fn delete_api_key(
    State(use_cases): State<Arc<ApiKeyUseCases>>,
    Path((tenant_id, key_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    // TODO: Verify that auth_user owns this tenant

    if let Err(e) = use_cases.delete(key_id, tenant_id).await {
        return error_response(
            StatusCode::BAD_REQUEST,
            "DELETE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "API_KEY_DELETED",
        "API key deleted successfully",
        serde_json::json!({"success": true}),
    )
}

// Helper function to map entity to response
fn map_to_response(key: ApiKeyEntity) -> ApiKeyResponse {
    ApiKeyResponse {
        id: key.id.to_string(),
        tenant_id: key.tenant_id.to_string(),
        key_prefix: key.key_prefix,
        name: key.name,
        environment: key.environment.unwrap_or_else(|| "development".to_string()),
        is_active: key.is_active.unwrap_or(true),
        last_used_at: key.last_used_at.map(|dt| dt.to_rfc3339()),
        created_at: key.created_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
    }
}
