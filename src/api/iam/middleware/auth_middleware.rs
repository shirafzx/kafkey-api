use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::application::use_cases::iam::AuthUseCases;

pub async fn auth_required<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Validate token
    let user = match auth_use_cases.validate_token(token).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Add user to request extensions
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}

pub async fn optional_auth<T: AuthUseCases>(
    State(auth_use_cases): State<Arc<T>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        });

    if let Some(token) = auth_header {
        // Validate token
        if let Ok(Some(user)) = auth_use_cases.validate_token(token).await {
            // Add user to request extensions
            request.extensions_mut().insert(user);
        }
    }

    next.run(request).await
}

// Helper to extract authenticated user from request extensions
pub fn extract_user(request: &Request) -> Option<&crate::domain::entities::iam::user::User> {
    request
        .extensions()
        .get::<crate::domain::entities::iam::user::User>()
}
