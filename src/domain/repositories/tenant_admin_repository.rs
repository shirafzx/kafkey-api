use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::tenant_admin::TenantAdminEntity;

#[async_trait]
pub trait TenantAdminRepository: Send + Sync {
    /// Create a new tenant admin
    async fn create(
        &self,
        email: String,
        password_hash: String,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<Uuid>;

    /// Find tenant admin by ID
    async fn find_by_id(&self, id: Uuid) -> Result<TenantAdminEntity>;

    /// Find tenant admin by email
    async fn find_by_email(&self, email: String) -> Result<TenantAdminEntity>;

    /// Update tenant admin
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<()>;

    /// Verify email
    async fn verify_email(&self, id: Uuid) -> Result<()>;

    /// Update password
    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()>;

    /// Deactivate tenant admin
    async fn deactivate(&self, id: Uuid) -> Result<()>;
}
