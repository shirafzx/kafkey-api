use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{
    permission::PermissionEntity,
    role::RoleEntity,
    user::{NewUserEntity, UserEntity},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: NewUserEntity) -> Result<Uuid>;
    async fn find_by_id(&self, id: Uuid) -> Result<UserEntity>;
    async fn find_by_username(&self, username: String) -> Result<UserEntity>;
    async fn find_by_email(&self, email: String) -> Result<UserEntity>;
    async fn update_last_login(&self, id: Uuid) -> Result<()>;
    async fn increment_failed_login(&self, id: Uuid) -> Result<()>;
    async fn reset_failed_login(&self, id: Uuid) -> Result<()>;
    async fn lock_account(&self, id: Uuid) -> Result<()>;

    // Role management
    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()>;
    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()>;
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<RoleEntity>>;
    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<PermissionEntity>>;

    async fn update_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        avatar_image_url: Option<String>,
    ) -> Result<()>;

    async fn admin_update(
        &self,
        user_id: Uuid,
        params: crate::domain::entities::user::AdminUpdateUserParams,
    ) -> Result<()>;

    async fn find_all(&self) -> Result<Vec<UserEntity>>;
    async fn find_paginated(&self, limit: i64, offset: i64) -> Result<Vec<UserEntity>>;
    async fn count(&self) -> Result<i64>;
    async fn delete(&self, id: Uuid) -> Result<()>;

    // Verification
    async fn find_by_verification_token(&self, token: String) -> Result<Option<UserEntity>>;
    async fn mark_as_verified(&self, id: Uuid) -> Result<()>;

    // Password Reset
    async fn find_by_password_reset_token(&self, token: String) -> Result<Option<UserEntity>>;
    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()>;

    // 2FA
    async fn update_2fa_status(
        &self,
        id: Uuid,
        secret: Option<String>,
        enabled: bool,
        backup_codes: Vec<Option<String>>,
    ) -> Result<()>;

    // Tenant-scoped methods for multi-tenancy
    async fn find_by_id_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<UserEntity>;
    async fn find_by_email_scoped(&self, email: String, tenant_id: Uuid) -> Result<UserEntity>;
    async fn find_all_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<UserEntity>>;
    async fn find_paginated_by_tenant(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserEntity>>;
    async fn count_by_tenant(&self, tenant_id: Uuid) -> Result<i64>;
    async fn delete_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<()>;
}
