use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::axum_http::dtos::{
        CreateRoleRequest, PermissionResponse, RoleResponse, UpdateRoleRequest,
    },
    api::axum_http::middleware::require_permission,
    application::use_cases::roles::RoleUseCases,
    infrastructure::database::postgres::repositories::role_repository::RolePostgres,
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
            "/api/v1/roles/:id",
            get(get_role_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/:id",
            put(update_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/:id",
            delete(delete_role).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.delete".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/:id/permissions",
            get(get_role_permissions).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/:id/permissions",
            post(assign_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/roles/:id/permissions/:permission_id",
            delete(remove_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("roles.update".to_string(), req, next)
            })),
        )
        .with_state(role_use_case)
}

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
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn create_role(
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    match role_use_case
        .create_role(request.name, request.description)
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_role_by_id(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case.get_role_by_id(id).await {
        Ok(r) => (
            StatusCode::OK,
            Json(RoleResponse {
                id: r.id.to_string(),
                name: r.name,
                description: r.description,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn update_role(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<UpdateRoleRequest>,
) -> impl IntoResponse {
    match role_use_case
        .update_role(id, request.name, request.description)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Role updated" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_role(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case.delete_role(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Role deleted" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
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
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Debug, serde::Deserialize)]
struct AssignPermissionRequest {
    permission_id: Uuid,
}

async fn assign_permission(
    Path(id): Path<Uuid>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
    Json(request): Json<AssignPermissionRequest>,
) -> impl IntoResponse {
    match role_use_case
        .assign_permission(id, request.permission_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Permission assigned" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn remove_permission(
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
    State(role_use_case): State<Arc<RoleUseCases<RolePostgres>>>,
) -> impl IntoResponse {
    match role_use_case
        .remove_permission(role_id, permission_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Permission removed" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
