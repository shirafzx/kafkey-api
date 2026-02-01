use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::tenant::TenantEntity;

#[async_trait]
pub trait TenantRepository: Send + Sync {
    /// Create a new tenant
    async fn create(
        &self,
        owner_id: Uuid,
        name: String,
        slug: String,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<Uuid>;

    /// Find tenant by ID
    async fn find_by_id(&self, id: Uuid) -> Result<TenantEntity>;

    /// Find tenant by slug
    async fn find_by_slug(&self, slug: String) -> Result<TenantEntity>;

    /// Get all tenants owned by a tenant admin
    async fn find_by_owner(&self, owner_id: Uuid) -> Result<Vec<TenantEntity>>;

    /// Update tenant
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<()>;

    /// Update plan tier
    async fn update_plan_tier(&self, id: Uuid, plan_tier: String, max_users: i32) -> Result<()>;

    /// Deactivate tenant
    async fn deactivate(&self, id: Uuid) -> Result<()>;

    /// Delete tenant
    async fn delete(&self, id: Uuid) -> Result<()>;
}
