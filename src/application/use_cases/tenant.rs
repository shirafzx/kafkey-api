use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::entities::tenant::TenantEntity,
    domain::repositories::{
        tenant_repository::TenantRepository, tenant_settings_repository::TenantSettingsRepository,
    },
};

pub struct TenantUseCases {
    tenant_repo: Arc<dyn TenantRepository>,
    tenant_settings_repo: Arc<dyn TenantSettingsRepository>,
}

impl TenantUseCases {
    pub fn new(
        tenant_repo: Arc<dyn TenantRepository>,
        tenant_settings_repo: Arc<dyn TenantSettingsRepository>,
    ) -> Self {
        Self {
            tenant_repo,
            tenant_settings_repo,
        }
    }

    /// Create a new tenant (application/organization)
    pub async fn create(
        &self,
        owner_id: Uuid,
        name: String,
        slug: String,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<Uuid> {
        // Validate slug format (alphanumeric and hyphens only)
        if !slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(anyhow::anyhow!(
                "Slug must contain only alphanumeric characters and hyphens"
            ));
        }

        // Create tenant
        let tenant_id = self
            .tenant_repo
            .create(owner_id, name.clone(), slug.clone(), domain, logo_url)
            .await?;

        // Create default settings for the tenant
        self.tenant_settings_repo.create(tenant_id).await?;

        tracing::info!(
            event = "TENANT_CREATED",
            tenant_id = %tenant_id,
            owner_id = %owner_id,
            name = %name,
            slug = %slug,
            "Tenant created successfully"
        );

        Ok(tenant_id)
    }

    /// Get tenant by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<TenantEntity> {
        self.tenant_repo.find_by_id(id).await
    }

    /// Get tenant by slug
    pub async fn get_by_slug(&self, slug: String) -> Result<TenantEntity> {
        self.tenant_repo.find_by_slug(slug).await
    }

    /// Get all tenants owned by a tenant admin
    pub async fn get_by_owner(&self, owner_id: Uuid) -> Result<Vec<TenantEntity>> {
        self.tenant_repo.find_by_owner(owner_id).await
    }

    /// Update tenant
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<()> {
        self.tenant_repo.update(id, name, domain, logo_url).await?;

        tracing::info!(
            event = "TENANT_UPDATED",
            tenant_id = %id,
            "Tenant updated successfully"
        );

        Ok(())
    }

    /// Update plan tier
    pub async fn update_plan_tier(
        &self,
        id: Uuid,
        plan_tier: String,
        max_users: i32,
    ) -> Result<()> {
        // Validate plan tier
        let valid_tiers = ["free", "starter", "professional", "enterprise"];
        if !valid_tiers.contains(&plan_tier.as_str()) {
            return Err(anyhow::anyhow!("Invalid plan tier"));
        }

        self.tenant_repo
            .update_plan_tier(id, plan_tier.clone(), max_users)
            .await?;

        tracing::info!(
            event = "TENANT_PLAN_UPDATED",
            tenant_id = %id,
            plan_tier = %plan_tier,
            max_users = max_users,
            "Tenant plan tier updated"
        );

        Ok(())
    }

    /// Deactivate tenant
    pub async fn deactivate(&self, id: Uuid) -> Result<()> {
        self.tenant_repo.deactivate(id).await?;

        tracing::info!(
            event = "TENANT_DEACTIVATED",
            tenant_id = %id,
            "Tenant deactivated"
        );

        Ok(())
    }

    /// Delete tenant (hard delete)
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.tenant_repo.delete(id).await?;

        tracing::info!(
            event = "TENANT_DELETED",
            tenant_id = %id,
            "Tenant deleted permanently"
        );

        Ok(())
    }
}
