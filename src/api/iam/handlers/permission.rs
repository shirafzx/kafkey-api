use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::application::use_cases::iam::permission_use_cases::{
    CheckPermissionRequest, CheckPermissionResponse, CreatePermissionRequest,
    CreatePermissionResponse, DeletePermissionRequest, DeletePermissionResponse,
    GetActionsForResourceRequest, GetActionsForResourceResponse, GetAllResourcesResponse,
    GetPermissionRequest, GetPermissionResponse, GetRolePermissionsResponse,
    GetUserPermissionsResponse, ListPermissionsRequest, ListPermissionsResponse,
    PermissionUseCases, UpdatePermissionRequest, UpdatePermissionResponse,
};

pub async fn create_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Json(request): Json<CreatePermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    match permission_use_cases.create_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Create permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetPermissionRequest {
        id: Some(id),
        name: None,
        resource: None,
        action: None,
    };

    match permission_use_cases.get_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get permission error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn get_permission_by_name<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetPermissionRequest {
        id: None,
        name: Some(name),
        resource: None,
        action: None,
    };

    match permission_use_cases.get_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get permission by name error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn get_permission_by_resource_action<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path((resource, action)): Path<(String, String)>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetPermissionRequest {
        id: None,
        name: None,
        resource: Some(resource),
        action: Some(action),
    };

    match permission_use_cases.get_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get permission by resource action error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
    Json(mut request): Json<UpdatePermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    request.id = id;

    match permission_use_cases.update_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Update permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = DeletePermissionRequest { id };

    match permission_use_cases.delete_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Delete permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_permissions<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Json(request): Json<Option<ListPermissionsRequest>>,
) -> Result<Json<Value>, StatusCode> {
    let request = request.unwrap_or_else(|| ListPermissionsRequest {
        limit: None,
        offset: None,
    });

    match permission_use_cases.list_permissions(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("List permissions error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_resources<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
) -> Result<Json<Value>, StatusCode> {
    match permission_use_cases.get_all_resources().await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get all resources error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_actions_for_resource<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(resource): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetActionsForResourceRequest { resource };

    match permission_use_cases.get_actions_for_resource(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get actions for resource error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn check_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(request): Json<CheckPermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = CheckPermissionRequest {
        user_id,
        resource: request.resource,
        action: request.action,
    };

    match permission_use_cases.check_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Check permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_permissions<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    match permission_use_cases.get_user_permissions(user_id).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user permissions error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_role_permissions<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    Path(role_id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    match permission_use_cases.get_role_permissions(role_id).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get role permissions error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
