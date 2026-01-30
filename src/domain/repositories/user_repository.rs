use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{
    permission::PermissionEntity,
    role::RoleEntity,
    user::{NewUserEntity, UserEntity},
};

#[async_trait]
pub trait UserRepository {
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
        display_name: Option<String>,
        avatar_image_url: Option<String>,
        is_active: Option<bool>,
        is_verified: Option<bool>,
    ) -> Result<()>;

    async fn find_all(&self) -> Result<Vec<UserEntity>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
