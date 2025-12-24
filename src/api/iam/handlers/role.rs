use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::application::use_cases::iam::role_use_cases::{
    AssignPermissionRequest, AssignPermissionResponse, AssignPermissionToUserRequest,
    AssignPermissionToUserResponse, CreateRoleRequest, CreateRoleResponse, DeleteRoleRequest,
    DeleteRoleResponse, GetRolePermissionsResponse, GetRoleRequest, GetRoleResponse,
    ListRolesRequest, ListRolesResponse, RevokePermissionRequest, RevokePermissionResponse,
    RoleUseCases, UpdateRoleRequest, UpdateRoleResponse,
};

pub async fn create_role<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<Json<Value>, StatusCode> {
    match role_use_cases.create_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Create role error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_role<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetRoleRequest {
        id: Some(id),
        name: None,
    };

    match role_use_cases.get_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get role error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn get_role_by_name<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(name): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetRoleRequest {
        id: None,
        name: Some(name),
    };

    match role_use_cases.get_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get role by name error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update_role<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
    Json(mut request): Json<UpdateRoleRequest>,
) -> Result<Json<Value>, StatusCode> {
    request.id = id;

    match role_use_cases.update_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Update role error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_role<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = DeleteRoleRequest { id };

    match role_use_cases.delete_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Delete role error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_roles<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Json(request): Json<Option<ListRolesRequest>>,
) -> Result<Json<Value>, StatusCode> {
    let request = request.unwrap_or_else(|| ListRolesRequest {
        limit: None,
        offset: None,
    });

    match role_use_cases.list_roles(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("List roles error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn assign_permission<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(role_id): Path<i32>,
    Json(request): Json<AssignPermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = AssignPermissionRequest {
        role_id,
        permission_id: request.permission_id,
    };

    match role_use_cases.assign_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Assign permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn revoke_permission<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(role_id): Path<i32>,
    Json(request): Json<RevokePermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = RevokePermissionRequest {
        role_id,
        permission_id: request.permission_id,
    };

    match role_use_cases.revoke_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Revoke permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_role_permissions<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(role_id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    match role_use_cases.get_role_permissions(role_id).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get role permissions error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn assign_permission_to_user<T: RoleUseCases>(
    State(role_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(request): Json<AssignPermissionToUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = AssignPermissionToUserRequest {
        user_id,
        resource: request.resource,
        action: request.action,
    };

    match role_use_cases.assign_permission_to_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Assign permission to user error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
