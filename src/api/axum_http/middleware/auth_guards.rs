use crate::services::jwt_service::TokenClaims;
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// Middleware to require a specific role
pub async fn require_role(
    required_role: String,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Authentication required").into_response())?;

    if claims.roles.iter().any(|r| r == &required_role) {
        Ok(next.run(request).await)
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Missing required role: {}", required_role),
        )
            .into_response())
    }
}

/// Middleware to require a specific permission
pub async fn require_permission(
    required_permission: String,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let claims = request
        .extensions()
        .get::<TokenClaims>()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Authentication required").into_response())?;

    if claims.permissions.iter().any(|p| p == &required_permission) {
        Ok(next.run(request).await)
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Missing required permission: {}", required_permission),
        )
            .into_response())
    }
}
