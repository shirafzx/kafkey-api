use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::use_cases::audit::AuditUseCases;
use crate::domain::repositories::audit_repository::AuditRepository;
use crate::domain::{
    entities::user::{AdminUpdateUserParams, NewUserEntity},
    repositories::user_repository::UserRepository,
};

pub struct UserUseCases<T, AR>
where
    T: UserRepository + Send + Sync,
    AR: AuditRepository + Send + Sync,
{
    user_repository: Arc<T>,
    audit_use_case: Arc<AuditUseCases<AR>>,
}

impl<T, AR> UserUseCases<T, AR>
where
    T: UserRepository + Send + Sync,
    AR: AuditRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T>, audit_use_case: Arc<AuditUseCases<AR>>) -> Self {
        Self {
            user_repository,
            audit_use_case,
        }
    }

    pub async fn create_user(&self, new_user: NewUserEntity) -> Result<Uuid> {
        let user_id = self.user_repository.create(new_user).await?;
        Ok(user_id)
    }

    pub async fn assign_default_role(
        &self,
        actor_id: Uuid,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<()> {
        self.user_repository.assign_role(user_id, role_id).await?;

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_USER_ROLE_ASSIGNED",
                Some(user_id),
                "user",
                "assign_role",
                serde_json::json!({ "role_id": role_id.to_string() }),
            )
            .await
            .ok();

        tracing::info!(
            audit = true,
            event = "AUDIT_USER_ROLE_ASSIGNED",
            actor_id = %actor_id,
            target_id = %user_id,
            role_id = %role_id,
            "Administrative action: Role assigned to user"
        );

        Ok(())
    }

    pub async fn get_user_by_id(
        &self,
        user_id: Uuid,
    ) -> Result<crate::domain::entities::user::UserEntity> {
        self.user_repository.find_by_id(user_id).await
    }

    pub async fn get_user_roles(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<crate::domain::entities::role::RoleEntity>> {
        self.user_repository.get_user_roles(user_id).await
    }

    pub async fn get_user_permissions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<crate::domain::entities::permission::PermissionEntity>> {
        self.user_repository.get_user_permissions(user_id).await
    }

    pub async fn update_user_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        avatar_image_url: Option<String>,
    ) -> Result<()> {
        self.user_repository
            .update_profile(user_id, display_name, avatar_image_url)
            .await
    }

    /// Get current user profile with roles - business logic for the GET /users/me endpoint
    pub async fn get_current_user_profile(
        &self,
        user_id_str: &str,
    ) -> Result<crate::application::dtos::UserProfileResponse> {
        // Parse user ID
        let user_id =
            Uuid::parse_str(user_id_str).map_err(|_| anyhow::anyhow!("Invalid user ID format"))?;

        // Get user from database
        let user = self.user_repository.find_by_id(user_id).await?;

        // Get user roles
        let roles = self
            .user_repository
            .get_user_roles(user_id)
            .await?
            .iter()
            .map(|r| r.name.clone())
            .collect();

        // Build response
        Ok(crate::application::dtos::UserProfileResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            avatar_image_url: user.avatar_image_url,
            is_active: user.is_active.unwrap_or(true),
            is_verified: user.is_verified.unwrap_or(false),
            roles,
            created_at: user.created_at.map(|dt| dt.to_rfc3339()),
        })
    }

    /// Update current user profile - business logic for the PUT /users/me endpoint
    pub async fn update_current_user_profile(
        &self,
        user_id_str: &str,
        display_name: Option<String>,
        avatar_image_url: Option<String>,
    ) -> Result<()> {
        // Parse user ID
        let user_id =
            Uuid::parse_str(user_id_str).map_err(|_| anyhow::anyhow!("Invalid user ID format"))?;

        // Update profile
        self.user_repository
            .update_profile(user_id, display_name, avatar_image_url)
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<crate::domain::entities::user::UserEntity>> {
        self.user_repository.find_all().await
    }

    pub async fn list_users_paginated(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<crate::domain::entities::user::UserEntity>, i64)> {
        let offset = (page - 1) * page_size;
        let items = self
            .user_repository
            .find_paginated(page_size, offset)
            .await?;
        let total = self.user_repository.count().await?;
        Ok((items, total))
    }

    pub async fn delete_user(&self, actor_id: Uuid, id: Uuid) -> Result<()> {
        self.user_repository.delete(id).await?;

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_USER_DELETED",
                Some(id),
                "user",
                "delete",
                serde_json::json!({}),
            )
            .await
            .ok();

        tracing::info!(
            audit = true,
            event = "AUDIT_USER_DELETED",
            actor_id = %actor_id,
            target_id = %id,
            "Administrative action: User deleted"
        );

        Ok(())
    }

    pub async fn admin_update_user(
        &self,
        actor_id: Uuid,
        user_id: Uuid,
        params: AdminUpdateUserParams,
    ) -> Result<()> {
        self.user_repository
            .admin_update(user_id, params.clone())
            .await?;

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_USER_UPDATED_ADMIN",
                Some(user_id),
                "user",
                "admin_update",
                serde_json::json!({
                    "display_name": params.display_name,
                    "is_active": params.is_active,
                    "is_verified": params.is_verified,
                }),
            )
            .await
            .ok();

        tracing::info!(
            audit = true,
            event = "AUDIT_USER_UPDATED_ADMIN",
            actor_id = %actor_id,
            target_id = %user_id,
            "Administrative action: User updated by admin"
        );

        Ok(())
    }

    pub async fn remove_role(&self, actor_id: Uuid, user_id: Uuid, role_id: Uuid) -> Result<()> {
        self.user_repository.remove_role(user_id, role_id).await?;

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_USER_ROLE_REMOVED",
                Some(user_id),
                "user",
                "remove_role",
                serde_json::json!({ "role_id": role_id.to_string() }),
            )
            .await
            .ok();

        tracing::info!(
            audit = true,
            event = "AUDIT_USER_ROLE_REMOVED",
            actor_id = %actor_id,
            target_id = %user_id,
            role_id = %role_id,
            "Administrative action: Role removed from user"
        );

        Ok(())
    }
}
