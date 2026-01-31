use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use uuid::Uuid;

use crate::{
    api::axum_http::middleware::{require_permission, require_role},
    api::axum_http::response_utils::{error_response, success_response},
    application::dtos::{
        AdminUpdateUserRequest, PaginatedResponse, PaginationParams, PermissionResponse,
        RoleResponse, UpdateProfileRequest, UserResponse,
    },
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
        .route(
            "/api/v1/users",
            get(list_users)
                .layer(axum::middleware::from_fn(|req, next| {
                    require_permission("users.read".to_string(), req, next)
                }))
                .layer(axum::middleware::from_fn(|req, next| {
                    require_role("admin".to_string(), req, next)
                })),
        )
        .route(
            "/api/v1/users/{id}",
            get(get_user_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}",
            put(admin_update_user).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}",
            delete(delete_user).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.delete".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}/roles",
            get(get_user_roles).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}/roles",
            post(assign_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}/roles/{role_id}",
            delete(remove_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("users.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/users/{id}/permissions",
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
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID in token",
                None,
            );
        }
    };

    match user_use_case.get_user_by_id(user_id).await {
        Ok(u) => success_response(
            "GET_PROFILE_SUCCESS",
            "User profile retrieved successfully",
            UserResponse {
                id: u.id.to_string(),
                username: u.username,
                email: u.email,
                display_name: u.display_name,
                avatar_image_url: u.avatar_image_url,
                is_active: u.is_active.unwrap_or(true),
                is_verified: u.is_verified.unwrap_or(false),
            },
        ),
        Err(e) => error_response(
            StatusCode::NOT_FOUND,
            "PROFILE_NOT_FOUND",
            &e.to_string(),
            None,
        ),
    }
}

async fn update_current_user(
    Extension(claims): Extension<TokenClaims>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
    Json(request): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_USER_ID",
                "Invalid user ID in token",
                None,
            );
        }
    };

    match user_use_case
        .update_user_profile(user_id, request.display_name, request.avatar_image_url)
        .await
    {
        Ok(_) => success_response(
            "UPDATE_PROFILE_SUCCESS",
            "User profile updated successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UPDATE_PROFILE_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
async fn list_users(
    Query(params): Query<PaginationParams>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    match user_use_case.list_users_paginated(page, page_size).await {
        Ok((users, total)) => {
            let user_responses: Vec<UserResponse> = users
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

            let total_pages = (total as f64 / page_size as f64).ceil() as i64;
            let has_next = page < total_pages;
            let has_prev = page > 1;

            let response = PaginatedResponse {
                items: user_responses,
                page,
                page_size,
                total_items: total,
                total_pages,
                has_next,
                has_prev,
            };

            success_response(
                "LIST_USERS_SUCCESS",
                "Users list retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "LIST_USERS_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn get_user_by_id(
    Path(id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.get_user_by_id(id).await {
        Ok(u) => success_response(
            "GET_USER_SUCCESS",
            "User retrieved successfully",
            UserResponse {
                id: u.id.to_string(),
                username: u.username,
                email: u.email,
                display_name: u.display_name,
                avatar_image_url: u.avatar_image_url,
                is_active: u.is_active.unwrap_or(true),
                is_verified: u.is_verified.unwrap_or(false),
            },
        ),
        Err(e) => error_response(
            StatusCode::NOT_FOUND,
            "USER_NOT_FOUND",
            &e.to_string(),
            None,
        ),
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
            None,
            None,
            None,
            None,
        )
        .await
    {
        Ok(_) => success_response(
            "ADMIN_UPDATE_USER_SUCCESS",
            "User updated by administrator successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ADMIN_UPDATE_USER_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn delete_user(
    Path(id): Path<Uuid>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.delete_user(id).await {
        Ok(_) => success_response(
            "DELETE_USER_SUCCESS",
            "User deleted successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_USER_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssignRoleRequest {
    role_id: Uuid,
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
        Ok(_) => success_response(
            "ASSIGN_ROLE_SUCCESS",
            "Role assigned successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ASSIGN_ROLE_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn remove_role(
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
    State(user_use_case): State<Arc<UserUseCases<UserPostgres>>>,
) -> impl IntoResponse {
    match user_use_case.remove_role(user_id, role_id).await {
        Ok(_) => success_response(
            "REMOVE_ROLE_SUCCESS",
            "Role removed successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "REMOVE_ROLE_FAILED",
            &e.to_string(),
            None,
        ),
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
            success_response(
                "GET_USER_ROLES_SUCCESS",
                "User roles retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "GET_USER_ROLES_FAILED",
            &e.to_string(),
            None,
        ),
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
            success_response(
                "GET_USER_PERMISSIONS_SUCCESS",
                "User permissions retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "GET_USER_PERMISSIONS_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
