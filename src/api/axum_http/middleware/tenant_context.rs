use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::repositories::api_key_repository::ApiKeyRepository,
    services::api_key_service::ApiKeyService,
};

/// Tenant context that will be added to request extensions
#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub api_key_id: Uuid,
}

/// Middleware to extract and validate tenant context from API key
pub async fn tenant_context_middleware(
    State(api_key_repo): State<Arc<dyn ApiKeyRepository>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract API key from Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Check if it's a Bearer token
    let api_key = if let Some(key) = auth_header.strip_prefix("Bearer ") {
        key
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Validate API key format
    if !ApiKeyService::validate_format(api_key) {
        tracing::warn!("Invalid API key format");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Hash the key to look it up
    let key_hash =
        ApiKeyService::hash_key(api_key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Find API key in database
    let api_key_entity = api_key_repo.find_by_hash(key_hash).await.map_err(|e| {
        tracing::warn!("API key not found or invalid: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    // Update last used timestamp (fire and forget)
    let repo_clone = api_key_repo.clone();
    let key_id = api_key_entity.id;
    tokio::spawn(async move {
        let _ = repo_clone.update_last_used(key_id).await;
    });

    // Create tenant context
    let tenant_context = TenantContext {
        tenant_id: api_key_entity.tenant_id,
        api_key_id: api_key_entity.id,
    };

    // Add to request extensions
    request.extensions_mut().insert(tenant_context);

    Ok(next.run(request).await)
}
