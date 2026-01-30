use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::{
    api::axum_http::dtos::UpdateProfileRequest,
    application::use_cases::users::UserUseCases,
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad, repositories::user_repository::UserPostgres,
    },
    services::jwt_service::TokenClaims,
};

use crate::api::axum_http::dtos::{
    AdminUpdateUserRequest, PermissionResponse, RoleResponse, UserResponse,
};
use crate::api::axum_http::middleware::require_permission;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let user_repository = Arc::new(UserPostgres::new(Arc::clone(&db_pool)));
    let user_use_case = Arc::new(UserUseCases::new(user_repository));

    Router::new()
        .route("/api/v1/users/me", get(get_current_user))
        .route("/api/v1/users/me", put(update_current_user))
        .route(
            "/api/v1/users",
            get(list_users).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id",
            get(get_user_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id",
            put(admin_update_user).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id",
            delete(delete_user).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.delete".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id/roles",
            get(get_user_roles).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id/roles",
            post(assign_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id/roles/:role_id",
            delete(remove_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/:id/permissions",
            get(get_user_permissions).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
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
async fn list_users(
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.list_users().await {
        Ok(users) => {
            let response: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id.to_string(),
                    username: u.username,
                    email: u.email,
                    display_name: u.display_name,
                    avatar_image_url: u.avatar_image_url,
                    is_active: u.is_active.unwrap_or(true),
                    is_verified: u.is_verified.unwrap_or(false),
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.get_user_by_id(id).await {
        Ok(user) => (
            StatusCode::OK,
            Json(UserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                display_name: user.display_name,
                avatar_image_url: user.avatar_image_url,
                is_active: user.is_active.unwrap_or(true),
                is_verified: user.is_verified.unwrap_or(false),
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn admin_update_user(
    Path(id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
    Json(request): Json<AdminUpdateUserRequest>,
) -> impl IntoResponse {
    match user_use_case
        .admin_update_user(
            id,
            request.display_name,
            request.avatar_image_url,
            request.is_active,
            request.is_verified,
        )
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "User updated successfully" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_user(
    Path(id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.delete_user(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "User deleted successfully" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: Uuid,
}

async fn assign_role(
    Path(user_id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
    Json(request): Json<AssignRoleRequest>,
) -> impl IntoResponse {
    match user_use_case
        .assign_default_role(user_id, request.role_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Role assigned successfully" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn remove_role(
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.remove_role(user_id, role_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Role removed successfully" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_user_roles(
    Path(user_id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.get_user_roles(user_id).await {
        Ok(roles) => {
            let response: Vec<RoleResponse> = roles
                .into_iter()
                .map(|r| RoleResponse {
                    id: r.id.to_string(),
                    name: r.name,
                    description: r.description,
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_user_permissions(
    Path(user_id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.get_user_permissions(user_id).await {
        Ok(perms) => {
            let response: Vec<PermissionResponse> = perms
                .into_iter()
                .map(|p| PermissionResponse {
                    id: p.id.to_string(),
                    name: p.name,
                    resource: p.resource,
                    action: p.action,
                    description: p.description,
                })
                .collect();
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
