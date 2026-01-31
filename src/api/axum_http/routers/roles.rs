use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::axum_http::middleware::require_permission,
    application::dtos::{CreateRoleRequest, PermissionResponse, RoleResponse, UpdateRoleRequest},
    application::use_cases::roles::RoleUseCases,
    infrastructure::database::postgres::repositories::role_repository::RolePostgres,
    services::jwt_service::TokenClaims,
};

pub fn routes(
    db_pool: Arc<crate::infrastructure::database::postgres::postgres_connection::PgPoolSquad>,
) -> Router {
    let role_repo = Arc::new(RolePostgres::new(db_pool));
    let role_use_case = Arc::new(RoleUseCases::new(role_repo));

    Router::new()
        .route(
            "/api/v1/roles",
            get(list_roles).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles",
            post(create_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.create".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}",
            get(get_role_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}",
            put(update_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}",
            delete(delete_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.delete".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}/permissions",
            get(get_role_permissions).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}/permissions",
            post(assign_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/{id}/permissions/{permission_id}",
            delete(remove_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .with_state(role_use_case)
}

use crate::api::axum_http::response_utils::{error_response, success_response};

async fn list_roles(
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case.list_roles().await {
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
                "LIST_ROLES_SUCCESS",
                "Roles list retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "LIST_ROLES_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn create_role(
    Extension(claims): Extension<TokenClaims>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    let actor_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_ACTOR_ID",
                "Invalid actor ID in token",
                None,
            );
        }
    };

    match role_use_case
        .create_role(actor_id, request.name, request.description)
        .await
    {
        Ok(id) => success_response(
            "CREATE_ROLE_SUCCESS",
            "Role created successfully",
            serde_json::json!({ "id": id }),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "CREATE_ROLE_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn get_role_by_id(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case.get_role_by_id(id).await {
        Ok(r) => success_response(
            "GET_ROLE_SUCCESS",
            "Role retrieved successfully",
            RoleResponse {
                id: r.id.to_string(),
                name: r.name,
                description: r.description,
            },
        ),
        Err(e) => error_response(
            StatusCode::NOT_FOUND,
            "ROLE_NOT_FOUND",
            &e.to_string(),
            None,
        ),
    }
}

async fn update_role(
    Extension(claims): Extension<TokenClaims>,
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<UpdateRoleRequest>,
) -> impl IntoResponse {
    let actor_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_ACTOR_ID",
                "Invalid actor ID in token",
                None,
            );
        }
    };

    match role_use_case
        .update_role(actor_id, id, request.name, request.description)
        .await
    {
        Ok(_) => success_response(
            "UPDATE_ROLE_SUCCESS",
            "Role updated successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UPDATE_ROLE_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn delete_role(
    Extension(claims): Extension<TokenClaims>,
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    let actor_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_ACTOR_ID",
                "Invalid actor ID in token",
                None,
            );
        }
    };

    match role_use_case.delete_role(actor_id, id).await {
        Ok(_) => success_response(
            "DELETE_ROLE_SUCCESS",
            "Role deleted successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_ROLE_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn get_role_permissions(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case.get_role_permissions(id).await {
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
                "GET_ROLE_PERMISSIONS_SUCCESS",
                "Role permissions retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "GET_ROLE_PERMISSIONS_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssignPermissionRequest {
    permission_id: Uuid,
}

async fn assign_permission(
    Extension(claims): Extension<TokenClaims>,
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<AssignPermissionRequest>,
) -> impl IntoResponse {
    let actor_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_ACTOR_ID",
                "Invalid actor ID in token",
                None,
            );
        }
    };

    match role_use_case
        .assign_permission(actor_id, id, request.permission_id)
        .await
    {
        Ok(_) => success_response(
            "ASSIGN_PERMISSION_SUCCESS",
            "Permission assigned to role successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "ASSIGN_PERMISSION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn remove_permission(
    Extension(claims): Extension<TokenClaims>,
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    let actor_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_ACTOR_ID",
                "Invalid actor ID in token",
                None,
            );
        }
    };

    match role_use_case
        .remove_permission(actor_id, role_id, permission_id)
        .await
    {
        Ok(_) => success_response(
            "REMOVE_PERMISSION_SUCCESS",
            "Permission removed from role successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "REMOVE_PERMISSION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
