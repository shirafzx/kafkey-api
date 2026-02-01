use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    api::axum_http::response_utils::{error_response, success_response},
    application::{
        dtos::{CreateTenantRequest, TenantResponse, UpdatePlanTierRequest, UpdateTenantRequest},
        use_cases::tenant::TenantUseCases,
    },
    domain::entities::tenant::TenantEntity,
    services::jwt_service::TokenClaims,
};

pub fn tenant_router(tenant_use_cases: Arc<TenantUseCases>) -> Router {
    Router::new()
        .route("/", post(create_tenant).get(list_my_tenants))
        .route(
            "/:id",
            get(get_tenant).put(update_tenant).delete(delete_tenant),
        )
        .route("/:id/plan", put(update_plan_tier))
        .route("/:id/deactivate", post(deactivate_tenant))
        .with_state(tenant_use_cases)
}

// Helper to parse user_id from TokenClaims
fn parse_user_id(claims: &TokenClaims) -> Result<Uuid, StatusCode> {
    Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)
}

/// Create a new tenant
async fn create_tenant(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Json(payload): Json<CreateTenantRequest>,
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

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    // Create tenant
    let tenant_id = match use_cases
        .create(
            user_id,
            payload.name,
            payload.slug,
            payload.domain,
            payload.logo_url,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "TENANT_CREATE_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    // Get the created tenant
    let tenant = match use_cases.get_by_id(tenant_id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "FETCH_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    success_response(
        "TENANT_CREATED",
        "Tenant created successfully",
        map_to_response(tenant),
    )
}

/// Get all tenants owned by the current tenant admin
async fn list_my_tenants(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
) -> impl IntoResponse {
    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    let tenants = match use_cases.get_by_owner(user_id).await {
        Ok(tenants) => tenants,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "FETCH_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    let responses: Vec<TenantResponse> = tenants.into_iter().map(map_to_response).collect();

    success_response(
        "TENANTS_FETCHED",
        "Tenants retrieved successfully",
        responses,
    )
}

/// Get tenant by ID
async fn get_tenant(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant = match use_cases.get_by_id(id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    // Verify ownership
    if tenant.owner_id != user_id {
        return error_response(
            StatusCode::FORBIDDEN,
            "ACCESS_DENIED",
            "You don't own this tenant",
            None,
        );
    }

    success_response(
        "TENANT_FETCHED",
        "Tenant retrieved successfully",
        map_to_response(tenant),
    )
}

/// Update tenant
async fn update_tenant(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantRequest>,
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

    // Verify ownership
    let tenant = match use_cases.get_by_id(id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    if tenant.owner_id != user_id {
        return error_response(
            StatusCode::FORBIDDEN,
            "ACCESS_DENIED",
            "You don't own this tenant",
            None,
        );
    }

    // Update
    if let Err(e) = use_cases
        .update(id, payload.name, payload.domain, payload.logo_url)
        .await
    {
        return error_response(
            StatusCode::BAD_REQUEST,
            "UPDATE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "TENANT_UPDATED",
        "Tenant updated successfully",
        serde_json::json!({"success": true}),
    )
}

/// Update plan tier
async fn update_plan_tier(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePlanTierRequest>,
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

    // Verify ownership
    let tenant = match use_cases.get_by_id(id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    if tenant.owner_id != user_id {
        return error_response(
            StatusCode::FORBIDDEN,
            "ACCESS_DENIED",
            "You don't own this tenant",
            None,
        );
    }

    // Update plan
    if let Err(e) = use_cases
        .update_plan_tier(id, payload.plan_tier, payload.max_users)
        .await
    {
        return error_response(
            StatusCode::BAD_REQUEST,
            "PLAN_UPDATE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "PLAN_UPDATED",
        "Plan tier updated successfully",
        serde_json::json!({"success": true}),
    )
}

/// Deactivate tenant
async fn deactivate_tenant(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Verify ownership
    let tenant = match use_cases.get_by_id(id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    if tenant.owner_id != user_id {
        return error_response(
            StatusCode::FORBIDDEN,
            "ACCESS_DENIED",
            "You don't own this tenant",
            None,
        );
    }

    // Deactivate
    if let Err(e) = use_cases.deactivate(id).await {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DEACTIVATE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "TENANT_DEACTIVATED",
        "Tenant deactivated successfully",
        serde_json::json!({"success": true}),
    )
}

/// Delete tenant (hard delete)
async fn delete_tenant(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantUseCases>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // Verify ownership
    let tenant = match use_cases.get_by_id(id).await {
        Ok(tenant) => tenant,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "TENANT_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    let user_id = match parse_user_id(&claims) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID",
                None,
            );
        }
    };

    if tenant.owner_id != user_id {
        return error_response(
            StatusCode::FORBIDDEN,
            "ACCESS_DENIED",
            "You don't own this tenant",
            None,
        );
    }

    // Delete
    if let Err(e) = use_cases.delete(id).await {
        return error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "TENANT_DELETED",
        "Tenant deleted successfully",
        serde_json::json!({"success": true}),
    )
}

// Helper function to map entity to response
fn map_to_response(tenant: TenantEntity) -> TenantResponse {
    TenantResponse {
        id: tenant.id.to_string(),
        owner_id: tenant.owner_id.to_string(),
        name: tenant.name,
        slug: tenant.slug,
        domain: tenant.domain,
        logo_url: tenant.logo_url,
        is_active: tenant.is_active.unwrap_or(true),
        plan_tier: tenant.plan_tier.unwrap_or_else(|| "free".to_string()),
        max_users: tenant.max_users.unwrap_or(1000),
        created_at: tenant
            .created_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
    }
}
