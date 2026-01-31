use std::sync::Arc;

use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    application::dtos::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest},
    application::use_cases::{auth::AuthUseCases, users::UserUseCases},
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            blacklist_repository::BlacklistPostgres, role_repository::RolePostgres,
            user_repository::UserPostgres,
        },
    },
    services::jwt_service::JwtService,
};

use crate::api::axum_http::response_utils::{error_response, success_response};

pub fn routes(db_pool: Arc<PgPoolSquad>, jwt_service: Arc<JwtService>) -> Router {
    let user_repository = Arc::new(UserPostgres::new(Arc::clone(&db_pool)));
    let role_repository = Arc::new(RolePostgres::new(Arc::clone(&db_pool)));

    let blacklist_repository = Arc::new(BlacklistPostgres::new(Arc::clone(&db_pool)));

    let user_use_case = Arc::new(UserUseCases::new(Arc::clone(&user_repository)));
    let auth_use_case = Arc::new(AuthUseCases::new(
        Arc::clone(&user_repository),
        Arc::clone(&role_repository),
        Arc::clone(&blacklist_repository),
        jwt_service,
    ));

    Router::new()
        .route("/api/v1/auth/sign-up", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh", post(refresh_token))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/verify-email", get(verify_email))
        .route(
            "/api/v1/auth/resend-verification",
            post(resend_verification),
        )
        .route("/api/v1/auth/forgot-password", post(forgot_password))
        .route("/api/v1/auth/reset-password", post(reset_password))
        .with_state((user_use_case, auth_use_case, role_repository))
}

async fn register(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth_use_case
        .register(
            request.username,
            request.email,
            request.display_name,
            request.password,
            request.avatar_image_url,
        )
        .await
    {
        Ok(user_id) => success_response(
            "REGISTER_SUCCESS",
            "User registered successfully",
            serde_json::json!({ "userId": user_id.to_string() }),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "REGISTER_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn login(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth_use_case
        .login(request.email_or_username, request.password)
        .await
    {
        Ok((user_id, access_token, refresh_token)) => success_response(
            "AUTH_SUCCESS",
            "Login successful",
            AuthResponse {
                user_id: user_id.to_string(),
                access_token,
                refresh_token,
            },
        ),
        Err(e) => error_response(
            StatusCode::UNAUTHORIZED,
            "AUTH_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn refresh_token(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    match auth_use_case.refresh_token(request.refresh_token).await {
        Ok(access_token) => success_response(
            "TOKEN_REFRESHED",
            "Access token refreshed successfully",
            serde_json::json!({ "accessToken": access_token }),
        ),
        Err(e) => error_response(
            StatusCode::UNAUTHORIZED,
            "REFRESH_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn logout(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    headers: axum::http::HeaderMap,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    let access_token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|h| h.to_string());

    let access_token = match access_token {
        Some(token) => token,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_AUTH",
                "Missing or invalid authorization header",
                None,
            );
        }
    };

    match auth_use_case
        .logout(access_token, Some(request.refresh_token))
        .await
    {
        Ok(_) => success_response(
            "LOGOUT_SUCCESS",
            "Logged out successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "LOGOUT_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn verify_email(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(t) => t.to_string(),
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_TOKEN",
                "Verification token is required",
                None,
            );
        }
    };

    match auth_use_case.verify_email(token).await {
        Ok(_) => success_response(
            "EMAIL_VERIFIED",
            "Email verified successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "VERIFICATION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResendVerificationRequest {
    email_or_username: String,
}

async fn resend_verification(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<ResendVerificationRequest>,
) -> impl IntoResponse {
    match auth_use_case
        .resend_verification_email(request.email_or_username)
        .await
    {
        Ok(_) => success_response(
            "VERIFICATION_SENT",
            "Verification email resent successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "RESEND_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForgotPasswordRequest {
    email: String,
}

async fn forgot_password(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<ForgotPasswordRequest>,
) -> impl IntoResponse {
    match auth_use_case.forgot_password(request.email).await {
        Ok(_) => success_response(
            "FORGOT_PASSWORD_SENT",
            "If the email exists, a reset link will be sent",
            serde_json::json!({}),
        ),
        Err(_) => {
            // Standard security practice: Don't reveal if email exists
            success_response(
                "FORGOT_PASSWORD_SENT",
                "If the email exists, a reset link will be sent",
                serde_json::json!({}),
            )
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResetPasswordRequest {
    token: String,
    new_password: String,
}

async fn reset_password(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres, BlacklistPostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    match auth_use_case
        .reset_password(request.token, request.new_password)
        .await
    {
        Ok(_) => success_response(
            "PASSWORD_RESET_SUCCESS",
            "Password has been reset successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "PASSWORD_RESET_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
