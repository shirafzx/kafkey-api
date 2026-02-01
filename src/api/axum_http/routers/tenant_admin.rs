use axum::{
    Extension, Json, Router,
    extract::State,
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
        dtos::{
            LoginTenantAdminRequest, LoginTenantAdminResponse, RegisterTenantAdminRequest,
            TenantAdminResponse, UpdatePasswordRequest, UpdateTenantAdminRequest,
        },
        use_cases::tenant_admin::TenantAdminUseCases,
    },
    domain::entities::tenant_admin::TenantAdminEntity,
    services::jwt_service::TokenClaims,
};

pub fn tenant_admin_router(tenant_admin_use_cases: Arc<TenantAdminUseCases>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_me).put(update_me))
        .route("/password", put(update_password))
        .with_state(tenant_admin_use_cases)
}

// Helper to parse user_id from TokenClaims
fn parse_user_id(claims: &TokenClaims) -> Result<Uuid, StatusCode> {
    Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)
}

/// Register a new tenant admin (service customer)
async fn register(
    State(use_cases): State<Arc<TenantAdminUseCases>>,
    Json(payload): Json<RegisterTenantAdminRequest>,
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

    // Register
    let admin_id = match use_cases
        .register(
            payload.email,
            payload.password,
            payload.name,
            payload.company_name,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "REGISTER_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    // Get the created admin
    let admin = match use_cases.get_by_id(admin_id).await {
        Ok(admin) => admin,
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
        "TENANT_ADMIN_REGISTERED",
        "Tenant admin registered successfully",
        map_to_response(admin),
    )
}

/// Login tenant admin
async fn login(
    State(use_cases): State<Arc<TenantAdminUseCases>>,
    Json(payload): Json<LoginTenantAdminRequest>,
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

    // Login
    let (token, admin) = match use_cases.login(payload.email, payload.password).await {
        Ok(result) => result,
        Err(e) => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "LOGIN_FAILED",
                &e.to_string(),
                None,
            );
        }
    };

    success_response(
        "LOGIN_SUCCESS",
        "Login successful",
        LoginTenantAdminResponse {
            access_token: token,
            admin: map_to_response(admin),
        },
    )
}

/// Get current tenant admin profile
async fn get_me(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantAdminUseCases>>,
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

    let admin = match use_cases.get_by_id(user_id).await {
        Ok(admin) => admin,
        Err(e) => {
            return error_response(
                StatusCode::NOT_FOUND,
                "ADMIN_NOT_FOUND",
                &e.to_string(),
                None,
            );
        }
    };

    success_response(
        "ADMIN_FETCHED",
        "Tenant admin profile retrieved",
        map_to_response(admin),
    )
}

/// Update tenant admin profile
async fn update_me(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantAdminUseCases>>,
    Json(payload): Json<UpdateTenantAdminRequest>,
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

    // Update
    if let Err(e) = use_cases
        .update_profile(user_id, payload.name, payload.company_name)
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
        "PROFILE_UPDATED",
        "Profile updated successfully",
        serde_json::json!({"success": true}),
    )
}

/// Update password
async fn update_password(
    Extension(claims): Extension<TokenClaims>,
    State(use_cases): State<Arc<TenantAdminUseCases>>,
    Json(payload): Json<UpdatePasswordRequest>,
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

    // Update password
    if let Err(e) = use_cases
        .update_password(user_id, payload.current_password, payload.new_password)
        .await
    {
        return error_response(
            StatusCode::BAD_REQUEST,
            "PASSWORD_UPDATE_FAILED",
            &e.to_string(),
            None,
        );
    }

    success_response(
        "PASSWORD_UPDATED",
        "Password updated successfully",
        serde_json::json!({"success": true}),
    )
}

// Helper function to map entity to response
fn map_to_response(admin: TenantAdminEntity) -> TenantAdminResponse {
    TenantAdminResponse {
        id: admin.id.to_string(),
        email: admin.email,
        name: admin.name,
        company_name: admin.company_name,
        is_active: admin.is_active.unwrap_or(true),
        email_verified: admin.email_verified.unwrap_or(false),
        created_at: admin
            .created_at
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(),
    }
}
