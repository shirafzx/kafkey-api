use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::api_key::ApiKeyEntity;

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    /// Create a new API key
    async fn create(
        &self,
        tenant_id: Uuid,
        key_hash: String,
        key_prefix: String,
        name: String,
        environment: Option<String>,
    ) -> Result<Uuid>;

    /// Find API key by hash
    async fn find_by_hash(&self, key_hash: String) -> Result<ApiKeyEntity>;

    /// Find all API keys for a tenant
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKeyEntity>>;

    /// Update last used timestamp
    async fn update_last_used(&self, id: Uuid) -> Result<()>;

    /// Revoke API key
    async fn revoke(&self, id: Uuid) -> Result<()>;

    /// Delete API key
    async fn delete(&self, id: Uuid) -> Result<()>;
}
