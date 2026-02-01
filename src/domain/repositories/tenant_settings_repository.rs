use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::entities::tenant_settings::TenantSettingsEntity;

#[async_trait]
pub trait TenantSettingsRepository: Send + Sync {
    /// Create default settings for a tenant
    async fn create(&self, tenant_id: Uuid) -> Result<Uuid>;

    /// Find settings by tenant ID
    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<TenantSettingsEntity>;

    /// Update settings
    async fn update(
        &self,
        tenant_id: Uuid,
        allow_signups: Option<bool>,
        require_email_verification: Option<bool>,
        enable_2fa: Option<bool>,
        session_duration_minutes: Option<i32>,
        allowed_oauth_providers: Option<Vec<String>>,
        webhook_url: Option<String>,
    ) -> Result<()>;

    /// Update custom email templates
    async fn update_email_templates(&self, tenant_id: Uuid, templates: JsonValue) -> Result<()>;
}
