use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::api::iam::middleware::auth_middleware::extract_user;
use crate::application::use_cases::iam::PermissionUseCases;

// Middleware to check if the authenticated user has a specific permission
pub async fn require_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    permission: (&'static str, &'static str), // (resource, action)
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract the authenticated user from the request
    let user = extract_user(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = user.id.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if the user has the required permission
    let has_permission = permission_use_cases
        .check_permission(
            crate::application::use_cases::iam::permission_use_cases::CheckPermissionRequest {
                user_id,
                resource: permission.0.to_string(),
                action: permission.1.to_string(),
            },
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .has_permission;

    if has_permission {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

// Middleware to check if the authenticated user has one of several permissions
pub async fn require_any_permission<T: PermissionUseCases>(
    State(permission_use_cases): State<Arc<T>>,
    permissions: Vec<(&'static str, &'static str)>, // [(resource, action)]
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract the authenticated user from the request
    let user = extract_user(&request).ok_or(StatusCode::UNAUTHORIZED)?;
    let user_id = user.id.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if the user has any of the required permissions
    for (resource, action) in permissions {
        let has_permission = permission_use_cases
            .check_permission(
                crate::application::use_cases::iam::permission_use_cases::CheckPermissionRequest {
                    user_id,
                    resource: resource.to_string(),
                    action: action.to_string(),
                },
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .has_permission;

        if has_permission {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::FORBIDDEN)
}

// Helper macro to create a permission middleware
#[macro_export]
macro_rules! require_permission {
    ($resource:expr, $action:expr) => {
        |State(permission_use_cases): State<Arc<impl PermissionUseCases>>,
         request: Request,
         next: Next| async {
            require_permission(permission_use_cases, ($resource, $action), request, next).await
        }
    };
}

// Helper macro to create a permission middleware for any of multiple permissions
#[macro_export]
macro_rules! require_any_permission {
    ($($resource:expr, $action:expr),*) => {
        |State(permission_use_cases): State<Arc<impl PermissionUseCases>>, request: Request, next: Next| async {
            let permissions = vec![$(($resource, $action)),*];
            require_any_permission(permission_use_cases, permissions, request, next).await
        }
    };
}
