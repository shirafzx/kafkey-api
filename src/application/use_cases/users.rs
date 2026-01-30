use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{entities::user::NewUserEntity, repositories::user_repository::UserRepository};

pub struct UserUseCases<T>
where
    T: UserRepository + Send + Sync,
{
    user_repository: Arc<T>,
}

impl<T> UserUseCases<T>
where
    T: UserRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, new_user: NewUserEntity) -> Result<Uuid> {
        let user_id = self.user_repository.create(new_user).await?;
        Ok(user_id)
    }

    pub async fn assign_default_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        self.user_repository.assign_role(user_id, role_id).await
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
}
