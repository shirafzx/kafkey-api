use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use uuid::Uuid;

use crate::{
    api::axum_http::dtos::{UpdateProfileRequest, UserProfileResponse},
    application::use_cases::users::UserUseCases,
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad, repositories::user_repository::UserPostgres,
    },
    services::jwt_service::TokenClaims,
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let user_repository = Arc::new(UserPostgres::new(Arc::clone(&db_pool)));
    let user_use_case = Arc::new(UserUseCases::new(user_repository));

    Router::new()
        .route("/api/v1/users/me", get(get_current_user))
        .route("/api/v1/users/me", put(update_current_user))
        .with_state(user_use_case)
}

async fn get_current_user(
    Extension(claims): Extension<TokenClaims>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    // Parse user ID from token claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user ID").into_response(),
    };

    // Get user from database
    let user = match user_use_case.get_user_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::NOT_FOUND, "User not found").into_response(),
    };

    // Get user roles
    let roles = match user_use_case.get_user_roles(user_id).await {
        Ok(roles) => roles.iter().map(|r| r.name.clone()).collect(),
        Err(_) => Vec::new(),
    };

    // Build response
    let response = UserProfileResponse {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
        display_name: user.display_name,
        avatar_image_url: user.avatar_image_url,
        is_active: user.is_active.unwrap_or(true),
        is_verified: user.is_verified.unwrap_or(false),
        roles,
        created_at: user.created_at.map(|dt| dt.to_rfc3339()),
    };

    (StatusCode::OK, Json(response)).into_response()
}

async fn update_current_user(
    Extension(claims): Extension<TokenClaims>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
    Json(request): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    // Parse user ID from token claims
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user ID").into_response(),
    };

    // Update user profile
    match user_use_case
        .update_user_profile(user_id, request.display_name, request.avatar_image_url)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Profile updated successfully"
            })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
