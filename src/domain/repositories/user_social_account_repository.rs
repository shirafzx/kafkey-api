use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use super::super::entities::user_social_account::{
    NewUserSocialAccountEntity, UpdateUserSocialAccountEntity, UserSocialAccountEntity,
};

#[async_trait]
pub trait UserSocialAccountRepository: Send + Sync {
    async fn create(
        &self,
        new_account: NewUserSocialAccountEntity,
    ) -> Result<UserSocialAccountEntity>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserSocialAccountEntity>>;

    async fn find_by_user_and_provider(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<Option<UserSocialAccountEntity>>;

    async fn find_by_provider_user(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserSocialAccountEntity>>;

    async fn find_all_by_user(&self, user_id: Uuid) -> Result<Vec<UserSocialAccountEntity>>;

    async fn update(
        &self,
        id: Uuid,
        update: UpdateUserSocialAccountEntity,
    ) -> Result<UserSocialAccountEntity>;

    async fn delete(&self, id: Uuid) -> Result<()>;
}
