use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::entities::api_key::ApiKeyEntity,
    domain::repositories::api_key_repository::ApiKeyRepository,
    services::api_key_service::ApiKeyService,
};

pub struct ApiKeyUseCases {
    api_key_repo: Arc<dyn ApiKeyRepository>,
}

impl ApiKeyUseCases {
    pub fn new(api_key_repo: Arc<dyn ApiKeyRepository>) -> Self {
        Self { api_key_repo }
    }

    /// Create a new API key for a tenant
    /// Returns (key_id, plain_key) - plain_key should be shown to user only once
    pub async fn create(
        &self,
        tenant_id: Uuid,
        name: String,
        environment: String,
    ) -> Result<(Uuid, String)> {
        // Generate API key
        let (plain_key, key_hash, key_prefix) = ApiKeyService::generate_key(&environment)?;

        // Store in database
        let key_id = self
            .api_key_repo
            .create(
                tenant_id,
                key_hash,
                key_prefix,
                name.clone(),
                Some(environment.clone()),
            )
            .await?;

        tracing::info!(
            event = "API_KEY_CREATED",
            key_id = %key_id,
            tenant_id = %tenant_id,
            name = %name,
            environment = %environment,
            "API key created successfully"
        );

        Ok((key_id, plain_key))
    }

    /// Get all API keys for a tenant
    pub async fn get_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKeyEntity>> {
        self.api_key_repo.find_by_tenant(tenant_id).await
    }

    /// Revoke an API key
    pub async fn revoke(&self, key_id: Uuid, tenant_id: Uuid) -> Result<()> {
        // First verify the key belongs to this tenant
        let key = self
            .api_key_repo
            .find_by_tenant(tenant_id)
            .await?
            .into_iter()
            .find(|k| k.id == key_id)
            .ok_or_else(|| {
                anyhow::anyhow!("API key not found or does not belong to this tenant")
            })?;

        // Revoke it
        self.api_key_repo.revoke(key.id).await?;

        tracing::info!(
            event = "API_KEY_REVOKED",
            key_id = %key_id,
            tenant_id = %tenant_id,
            "API key revoked"
        );

        Ok(())
    }

    /// Delete an API key
    pub async fn delete(&self, key_id: Uuid, tenant_id: Uuid) -> Result<()> {
        // First verify the key belongs to this tenant
        let key = self
            .api_key_repo
            .find_by_tenant(tenant_id)
            .await?
            .into_iter()
            .find(|k| k.id == key_id)
            .ok_or_else(|| {
                anyhow::anyhow!("API key not found or does not belong to this tenant")
            })?;

        // Delete it
        self.api_key_repo.delete(key.id).await?;

        tracing::info!(
            event = "API_KEY_DELETED",
            key_id = %key_id,
            tenant_id = %tenant_id,
            "API key deleted permanently"
        );

        Ok(())
    }

    /// Validate an API key (used by middleware)
    pub async fn validate(&self, plain_key: String) -> Result<ApiKeyEntity> {
        // Validate format
        if !ApiKeyService::validate_format(&plain_key) {
            return Err(anyhow::anyhow!("Invalid API key format"));
        }

        // Hash the key
        let key_hash = ApiKeyService::hash_key(&plain_key)?;

        // Find in database
        let api_key = self.api_key_repo.find_by_hash(key_hash).await?;

        Ok(api_key)
    }
}
