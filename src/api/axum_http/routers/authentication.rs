use std::sync::Arc;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    api::axum_http::dtos::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest},
    application::use_cases::{auth_use_cases::AuthUseCases, users::UserUseCases},
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            blacklist_repository::BlacklistPostgres, role_repository::RolePostgres,
            user_repository::UserPostgres,
        },
    },
    services::jwt_service::JwtService,
};

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
        Ok(user_id) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "User registered successfully",
                "user_id": user_id.to_string()
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
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
        Ok((user_id, access_token, refresh_token)) => (
            StatusCode::OK,
            Json(AuthResponse {
                user_id: user_id.to_string(),
                access_token,
                refresh_token,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
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
        Ok(access_token) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "access_token": access_token
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
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
            return (
                StatusCode::BAD_REQUEST,
                "Missing or invalid authorization header",
            )
                .into_response();
        }
    };

    match auth_use_case
        .logout(access_token, Some(request.refresh_token))
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Logged out successfully"
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
