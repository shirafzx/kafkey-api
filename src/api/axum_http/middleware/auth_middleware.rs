use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::{
    domain::repositories::blacklist_repository::BlacklistRepository,
    services::jwt_service::{JwtService, TokenClaims},
};
use uuid::Uuid;

/// Authentication middleware that validates JWT tokens
pub async fn auth_middleware<B>(
    jwt_service: Arc<JwtService>,
    blacklist_repository: Arc<B>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, Response>
where
    B: BlacklistRepository + Send + Sync,
{
    // Extract authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ").to_string()
        }
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            )
                .into_response());
        }
    };

    // Validate token
    let claims = match jwt_service.validate_access_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response());
        }
    };

    // Check if blacklisted
    let jti = Uuid::parse_str(&claims.jti)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token identifier").into_response())?;

    if blacklist_repository
        .is_blacklisted(jti)
        .await
        .unwrap_or(false)
    {
        return Err((StatusCode::UNAUTHORIZED, "Token has been revoked").into_response());
    }

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

/// Extractor for authenticated user claims
/// Use this in route handlers to access the authenticated user's information
#[derive(Clone, Debug)]
pub struct AuthUser(pub TokenClaims);

impl AuthUser {
    pub fn user_id(&self) -> &str {
        &self.0.sub
    }

    pub fn roles(&self) -> &[String] {
        &self.0.roles
    }

    pub fn permissions(&self) -> &[String] {
        &self.0.permissions
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.0.roles.iter().any(|r| r == role)
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.0.permissions.iter().any(|p| p == permission)
    }
}

// Helper to extract AuthUser from request extensions
impl axum::extract::FromRef<TokenClaims> for AuthUser {
    fn from_ref(claims: &TokenClaims) -> Self {
        AuthUser(claims.clone())
    }
}
