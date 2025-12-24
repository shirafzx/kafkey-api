use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::application::use_cases::iam::user_use_cases::{
    AssignRoleRequest, AssignRoleResponse, CheckPermissionRequest, CheckPermissionResponse,
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    GetUserPermissionsResponse, GetUserRequest, GetUserResponse, GetUserRolesResponse,
    ListUsersRequest, ListUsersResponse, RevokeRoleRequest, RevokeRoleResponse, UpdateUserRequest,
    UpdateUserResponse, UserUseCases,
};

pub async fn create_user<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    match user_use_cases.create_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Create user error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetUserRequest {
        id: Some(id),
        username: None,
        email: None,
    };

    match user_use_cases.get_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn get_user_by_username<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(username): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetUserRequest {
        id: None,
        username: Some(username),
        email: None,
    };

    match user_use_cases.get_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user by username error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn get_user_by_email<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(email): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let request = GetUserRequest {
        id: None,
        username: None,
        email: Some(email),
    };

    match user_use_cases.get_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user by email error: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update_user<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
    Json(mut request): Json<UpdateUserRequest>,
) -> Result<Json<Value>, StatusCode> {
    request.id = id;

    match user_use_cases.update_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Update user error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let request = DeleteUserRequest { id };

    match user_use_cases.delete_user(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Delete user error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_users<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Json(request): Json<Option<ListUsersRequest>>,
) -> Result<Json<Value>, StatusCode> {
    let request = request.unwrap_or_else(|| ListUsersRequest {
        limit: None,
        offset: None,
    });

    match user_use_cases.list_users(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("List users error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn assign_role<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(request): Json<AssignRoleRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = AssignRoleRequest {
        user_id,
        role_id: request.role_id,
    };

    match user_use_cases.assign_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Assign role error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn revoke_role<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(request): Json<RevokeRoleRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = RevokeRoleRequest {
        user_id,
        role_id: request.role_id,
    };

    match user_use_cases.revoke_role(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Revoke role error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn check_permission<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
    Json(request): Json<CheckPermissionRequest>,
) -> Result<Json<Value>, StatusCode> {
    let request = CheckPermissionRequest {
        user_id,
        resource: request.resource,
        action: request.action,
    };

    match user_use_cases.check_permission(request).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Check permission error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_roles<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    match user_use_cases.get_user_roles(user_id).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user roles error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user_permissions<T: UserUseCases>(
    State(user_use_cases): State<Arc<T>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    match user_use_cases.get_user_permissions(user_id).await {
        Ok(response) => Ok(Json(json!(response))),
        Err(e) => {
            tracing::error!("Get user permissions error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
