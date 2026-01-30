use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::axum_http::dtos::{CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest},
    api::axum_http::middleware::require_permission,
    application::use_cases::permissions::PermissionUseCases,
    infrastructure::database::postgres::repositories::permission_repository::PermissionPostgres,
};
use axum::routing::{delete, post, put};

pub fn routes(
    db_pool: Arc<crate::infrastructure::database::postgres::postgres_connection::PgPoolSquad>,
) -> Router {
    let permission_repo = Arc::new(PermissionPostgres::new(db_pool));
    let permission_use_case = Arc::new(PermissionUseCases::new(permission_repo));

    Router::new()
        .route(
            "/api/v1/permissions",
            get(list_permissions).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions",
            post(create_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.create".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions/:id",
            get(get_permission_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions/:id",
            put(update_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions/:id",
            delete(delete_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.delete".to_string(), req, next)
            })),
        )
        .with_state(permission_use_case)
}

async fn list_permissions(
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
) -> impl IntoResponse {
    match permission_use_case.list_permissions().await {
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

async fn create_permission(
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
    Json(request): Json<CreatePermissionRequest>,
) -> impl IntoResponse {
    match permission_use_case
        .create_permission(
            request.name,
            request.resource,
            request.action,
            request.description,
        )
        .await
    {
        Ok(id) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn get_permission_by_id(
    Path(id): Path<Uuid>,
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
) -> impl IntoResponse {
    match permission_use_case.get_permission_by_id(id).await {
        Ok(p) => (
            StatusCode::OK,
            Json(PermissionResponse {
                id: p.id.to_string(),
                name: p.name,
                resource: p.resource,
                action: p.action,
                description: p.description,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

async fn update_permission(
    Path(id): Path<Uuid>,
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
    Json(request): Json<UpdatePermissionRequest>,
) -> impl IntoResponse {
    match permission_use_case
        .update_permission(id, request.name, request.description)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Permission updated" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn delete_permission(
    Path(id): Path<Uuid>,
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
) -> impl IntoResponse {
    match permission_use_case.delete_permission(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Permission deleted" })),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
