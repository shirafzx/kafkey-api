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
    api::axum_http::middleware::require_permission,
    application::dtos::{CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest},
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
            "/api/v1/permissions/{id}",
            get(get_permission_by_id).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.read".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions/{id}",
            put(update_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.update".to_string(), req, next)
            })),
        )
        .route(
            "/api/v1/permissions/{id}",
            delete(delete_permission).layer(axum::middleware::from_fn(|req, next| {
                require_permission("permissions.delete".to_string(), req, next)
            })),
        )
        .with_state(permission_use_case)
}

use crate::api::axum_http::response_utils::{error_response, success_response};

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
            success_response(
                "LIST_PERMISSIONS_SUCCESS",
                "Permissions list retrieved successfully",
                response,
            )
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "LIST_PERMISSIONS_FAILED",
            &e.to_string(),
            None,
        ),
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
        Ok(id) => success_response(
            "CREATE_PERMISSION_SUCCESS",
            "Permission created successfully",
            serde_json::json!({ "id": id }),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "CREATE_PERMISSION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn get_permission_by_id(
    Path(id): Path<Uuid>,
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
) -> impl IntoResponse {
    match permission_use_case.get_permission_by_id(id).await {
        Ok(p) => success_response(
            "GET_PERMISSION_SUCCESS",
            "Permission retrieved successfully",
            PermissionResponse {
                id: p.id.to_string(),
                name: p.name,
                resource: p.resource,
                action: p.action,
                description: p.description,
            },
        ),
        Err(e) => error_response(
            StatusCode::NOT_FOUND,
            "PERMISSION_NOT_FOUND",
            &e.to_string(),
            None,
        ),
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
        Ok(_) => success_response(
            "UPDATE_PERMISSION_SUCCESS",
            "Permission updated successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UPDATE_PERMISSION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

async fn delete_permission(
    Path(id): Path<Uuid>,
    State(permission_use_case): State<Arc<PermissionUseCases<PermissionPostgres>>>,
) -> impl IntoResponse {
    match permission_use_case.delete_permission(id).await {
        Ok(_) => success_response(
            "DELETE_PERMISSION_SUCCESS",
            "Permission deleted successfully",
            serde_json::json!({}),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_PERMISSION_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
