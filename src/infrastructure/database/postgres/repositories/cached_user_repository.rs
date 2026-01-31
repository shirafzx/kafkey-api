use anyhow::{Ok, Result};
use async_trait::async_trait;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use crate::domain::{
    entities::{
        permission::PermissionEntity,
        role::RoleEntity,
        user::{AdminUpdateUserParams, NewUserEntity, UserEntity},
    },
    repositories::user_repository::UserRepository,
};

pub struct CachedUserRepository<T> {
    inner: Arc<T>,
    permissions_cache: Cache<Uuid, Vec<PermissionEntity>>,
}

impl<T> CachedUserRepository<T>
where
    T: UserRepository + Send + Sync + 'static,
{
    pub fn new(inner: Arc<T>) -> Self {
        // Cache configuration:
        // - Max capacity: 10,000 entries
        // - Time to live: 5 minutes (permissions don't change often, but we want reasonable freshness)
        let permissions_cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(300))
            .build();

        Self {
            inner,
            permissions_cache,
        }
    }

    /// Invalidate cache for a specific user
    async fn invalidate_permissions(&self, user_id: Uuid) {
        self.permissions_cache.remove(&user_id).await;
    }
}

#[async_trait]
impl<T> UserRepository for CachedUserRepository<T>
where
    T: UserRepository + Send + Sync + 'static,
{
    // Pass-through methods for standard CRUD
    async fn create(&self, new_user: NewUserEntity) -> Result<Uuid> {
        self.inner.create(new_user).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<UserEntity> {
        self.inner.find_by_id(id).await
    }

    async fn find_by_username(&self, username: String) -> Result<UserEntity> {
        self.inner.find_by_username(username).await
    }

    async fn find_by_email(&self, email: String) -> Result<UserEntity> {
        self.inner.find_by_email(email).await
    }

    async fn update_last_login(&self, id: Uuid) -> Result<()> {
        self.inner.update_last_login(id).await
    }

    async fn increment_failed_login(&self, id: Uuid) -> Result<()> {
        self.inner.increment_failed_login(id).await
    }

    async fn reset_failed_login(&self, id: Uuid) -> Result<()> {
        self.inner.reset_failed_login(id).await
    }

    async fn lock_account(&self, id: Uuid) -> Result<()> {
        self.inner.lock_account(id).await
    }

    // Role management - Write Side (Invalidates Cache)
    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let result = self.inner.assign_role(user_id, role_id).await;
        if result.is_ok() {
            self.invalidate_permissions(user_id).await;
        }
        result
    }

    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let result = self.inner.remove_role(user_id, role_id).await;
        if result.is_ok() {
            self.invalidate_permissions(user_id).await;
        }
        result
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<RoleEntity>> {
        self.inner.get_user_roles(user_id).await
    }

    // Cached Read
    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<PermissionEntity>> {
        // Try getting from cache first
        if let Some(permissions) = self.permissions_cache.get(&user_id).await {
            tracing::debug!("Cache HIT: Permissions for user {}", user_id);
            return Ok(permissions);
        }

        tracing::debug!("Cache MISS: Permissions for user {}", user_id);

        // Cache miss: fetch from DB
        let permissions = self.inner.get_user_permissions(user_id).await?;

        // Store in cache
        self.permissions_cache
            .insert(user_id, permissions.clone())
            .await;

        Ok(permissions)
    }

    async fn update_profile(
        &self,
        user_id: Uuid,
        display_name: Option<String>,
        avatar_image_url: Option<String>,
    ) -> Result<()> {
        self.inner
            .update_profile(user_id, display_name, avatar_image_url)
            .await
    }

    async fn admin_update(&self, user_id: Uuid, params: AdminUpdateUserParams) -> Result<()> {
        self.inner.admin_update(user_id, params).await
    }

    async fn find_all(&self) -> Result<Vec<UserEntity>> {
        self.inner.find_all().await
    }

    async fn find_paginated(&self, limit: i64, offset: i64) -> Result<Vec<UserEntity>> {
        self.inner.find_paginated(limit, offset).await
    }

    async fn count(&self) -> Result<i64> {
        self.inner.count().await
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        self.inner.delete(id).await
    }

    async fn find_by_verification_token(&self, token: String) -> Result<Option<UserEntity>> {
        self.inner.find_by_verification_token(token).await
    }

    async fn mark_as_verified(&self, id: Uuid) -> Result<()> {
        self.inner.mark_as_verified(id).await
    }

    async fn find_by_password_reset_token(&self, token: String) -> Result<Option<UserEntity>> {
        self.inner.find_by_password_reset_token(token).await
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()> {
        self.inner.update_password(id, password_hash).await
    }

    async fn update_2fa_status(
        &self,
        id: Uuid,
        secret: Option<String>,
        enabled: bool,
        backup_codes: Vec<Option<String>>,
    ) -> Result<()> {
        self.inner
            .update_2fa_status(id, secret, enabled, backup_codes)
            .await
    }
}
