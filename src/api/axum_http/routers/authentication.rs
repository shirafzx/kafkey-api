use std::sync::Arc;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    api::axum_http::dtos::{AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest},
    application::use_cases::{auth_use_cases::AuthUseCases, users::UserUseCases},
    domain::{
        entities::user::NewUserEntity,
        repositories::{role_repository::RoleRepository, user_repository::UserRepository},
    },
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            permission_repository::PermissionPostgres, role_repository::RolePostgres,
            user_repository::UserPostgres,
        },
    },
    services::{jwt_service::JwtService, password_service::PasswordService},
};

pub fn routes(db_pool: Arc<PgPoolSquad>, jwt_service: Arc<JwtService>) -> Router {
    let user_repository = Arc::new(UserPostgres::new(Arc::clone(&db_pool)));
    let role_repository = Arc::new(RolePostgres::new(Arc::clone(&db_pool)));

    let user_use_case = Arc::new(UserUseCases::new(Arc::clone(&user_repository)));
    let auth_use_case = Arc::new(AuthUseCases::new(
        Arc::clone(&user_repository),
        Arc::clone(&role_repository),
        jwt_service,
    ));

    Router::new()
        .route("/api/v1/auth/sign-up", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh", post(refresh_token))
        .with_state((user_use_case, auth_use_case, role_repository))
}

async fn register(
    axum::extract::State((user_use_case, _, role_repo)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres>>,
        Arc<RolePostgres>,
    )>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Hash password
    let password_hash = match PasswordService::hash_password(&request.password) {
        Ok(hash) => hash,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // Create user entity
    let new_user = NewUserEntity {
        username: request.username,
        email: request.email,
        display_name: request.display_name,
        avatar_image_url: request.avatar_image_url,
        password_hash,
    };

    // Create user
    let user_id = match user_use_case.create_user(new_user).await {
        Ok(id) => id,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // Assign default "user" role
    match role_repo.find_by_name("user".to_string()).await {
        Ok(role) => {
            if let Err(e) = user_use_case.assign_default_role(user_id, role.id).await {
                tracing::warn!("Failed to assign default role: {}", e);
            }
        }
        Err(e) => tracing::warn!("Failed to find default role: {}", e),
    }

    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "message": "User registered successfully",
            "user_id": user_id.to_string()
        })),
    )
        .into_response()
}

async fn login(
    axum::extract::State((_, auth_use_case, _)): axum::extract::State<(
        Arc<UserUseCases<UserPostgres>>,
        Arc<AuthUseCases<UserPostgres, RolePostgres>>,
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
        Arc<AuthUseCases<UserPostgres, RolePostgres>>,
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
