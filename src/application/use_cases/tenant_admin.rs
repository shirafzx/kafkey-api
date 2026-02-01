use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::entities::tenant_admin::TenantAdminEntity,
    domain::repositories::tenant_admin_repository::TenantAdminRepository,
    services::{jwt_service::JwtService, password_service::PasswordService},
};

pub struct TenantAdminUseCases {
    tenant_admin_repo: Arc<dyn TenantAdminRepository>,
    jwt_service: Arc<JwtService>,
}

impl TenantAdminUseCases {
    pub fn new(
        tenant_admin_repo: Arc<dyn TenantAdminRepository>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            tenant_admin_repo,
            jwt_service,
        }
    }

    /// Register a new tenant admin (service customer)
    pub async fn register(
        &self,
        email: String,
        password: String,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<Uuid> {
        // Hash password
        let password_hash = PasswordService::hash_password(&password)?;

        // Create tenant admin
        let admin_id = self
            .tenant_admin_repo
            .create(email.clone(), password_hash, name, company_name)
            .await?;

        tracing::info!(
            event = "TENANT_ADMIN_REGISTERED",
            admin_id = %admin_id,
            email = %email,
            "Tenant admin registered successfully"
        );

        Ok(admin_id)
    }

    /// Login tenant admin and generate JWT
    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<(String, TenantAdminEntity)> {
        // Find admin by email
        let admin = self.tenant_admin_repo.find_by_email(email.clone()).await?;

        // Check if active
        if !admin.is_active.unwrap_or(true) {
            return Err(anyhow::anyhow!("Account is deactivated"));
        }

        // Verify password
        let is_valid = PasswordService::verify_password(&password, &admin.password_hash)?;
        if !is_valid {
            tracing::warn!(
                event = "TENANT_ADMIN_LOGIN_FAILED",
                email = %email,
                "Invalid credentials"
            );
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Generate JWT (using admin role)
        let token = self.jwt_service.generate_access_token(
            admin.id,
            vec!["tenant_admin".to_string()],
            vec!["tenants.*".to_string(), "api_keys.*".to_string()],
            None,
        )?;

        tracing::info!(
            event = "TENANT_ADMIN_LOGIN_SUCCESS",
            admin_id = %admin.id,
            "Tenant admin logged in successfully"
        );

        Ok((token, admin))
    }

    /// Get tenant admin by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<TenantAdminEntity> {
        self.tenant_admin_repo.find_by_id(id).await
    }

    /// Update tenant admin profile
    pub async fn update_profile(
        &self,
        id: Uuid,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<()> {
        self.tenant_admin_repo
            .update(id, name, company_name)
            .await?;

        tracing::info!(
            event = "TENANT_ADMIN_UPDATED",
            admin_id = %id,
            "Tenant admin profile updated"
        );

        Ok(())
    }

    /// Update password
    pub async fn update_password(
        &self,
        id: Uuid,
        current_password: String,
        new_password: String,
    ) -> Result<()> {
        // Get admin
        let admin = self.tenant_admin_repo.find_by_id(id).await?;

        // Verify current password
        let is_valid = PasswordService::verify_password(&current_password, &admin.password_hash)?;
        if !is_valid {
            return Err(anyhow::anyhow!("Current password is incorrect"));
        }

        // Hash new password
        let new_hash = PasswordService::hash_password(&new_password)?;

        // Update
        self.tenant_admin_repo.update_password(id, new_hash).await?;

        tracing::info!(
            event = "TENANT_ADMIN_PASSWORD_UPDATED",
            admin_id = %id,
            "Tenant admin password updated"
        );

        Ok(())
    }
}
