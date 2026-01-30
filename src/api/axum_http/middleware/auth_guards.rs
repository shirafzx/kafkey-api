use crate::{api::axum_http::response_utils::error_response, services::jwt_service::TokenClaims};
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};

/// Middleware to require a specific role
pub async fn require_role(
    required_role: String,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let claims = request.extensions().get::<TokenClaims>().ok_or_else(|| {
        error_response(
            StatusCode::UNAUTHORIZED,
            "AUTH_REQUIRED",
            "Authentication required",
            None,
        )
    })?;

    if claims.roles.iter().any(|r| r == &required_role) {
        Ok(next.run(request).await)
    } else {
        Err(error_response(
            StatusCode::FORBIDDEN,
            "MISSING_ROLE",
            &format!("Missing required role: {}", required_role),
            None,
        ))
    }
}

/// Middleware to require a specific permission
pub async fn require_permission(
    required_permission: String,
    request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let claims = request.extensions().get::<TokenClaims>().ok_or_else(|| {
        error_response(
            StatusCode::UNAUTHORIZED,
            "AUTH_REQUIRED",
            "Authentication required",
            None,
        )
    })?;

    if claims.permissions.iter().any(|p| p == &required_permission) {
        Ok(next.run(request).await)
    } else {
        Err(error_response(
            StatusCode::FORBIDDEN,
            "MISSING_PERMISSION",
            &format!("Missing required permission: {}", required_permission),
            None,
        ))
    }
}
