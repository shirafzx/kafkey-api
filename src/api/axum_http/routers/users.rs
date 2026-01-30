use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};

use crate::{
    api::axum_http::dtos::UpdateProfileRequest,
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
    match user_use_case.get_current_user_profile(&claims.sub).await {
        Ok(profile) => (StatusCode::OK, Json(profile)).into_response(),
        Err(e) if e.to_string().contains("Invalid user ID") => {
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
        Err(e) if e.to_string().contains("not found") => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn update_current_user(
    Extension(claims): Extension<TokenClaims>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
    Json(request): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    match user_use_case
        .update_current_user_profile(&claims.sub, request.display_name, request.avatar_image_url)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Profile updated successfully"
            })),
        )
            .into_response(),
        Err(e) if e.to_string().contains("Invalid user ID") => {
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
