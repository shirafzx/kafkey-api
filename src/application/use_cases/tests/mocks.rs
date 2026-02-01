use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::mock;
use uuid::Uuid;

use crate::domain::entities::{
    audit_log::NewAuditLogEntity,
    permission::{NewPermissionEntity, PermissionEntity},
    role::{NewRoleEntity, RoleEntity},
    user::{AdminUpdateUserParams, NewUserEntity, UserEntity},
    user_social_account::{NewUserSocialAccountEntity, UserSocialAccountEntity},
};
use crate::domain::repositories::{
    audit_repository::AuditRepository, blacklist_repository::BlacklistRepository,
    permission_repository::PermissionRepository, role_repository::RoleRepository,
    user_repository::UserRepository, user_social_account_repository::UserSocialAccountRepository,
};

// Mock AuditRepository
mock! {
    pub AuditRepo {}

    #[async_trait]
    impl AuditRepository for AuditRepo {
        async fn create(&self, new_audit: NewAuditLogEntity) -> Result<()>;
        async fn find_all(&self) -> Result<Vec<crate::domain::entities::audit_log::AuditLogEntity>>;
    }
}

// Mock UserRepository
mock! {
    pub UserRepo {}

    #[async_trait]
    impl UserRepository for UserRepo {
        async fn create(&self, new_user: NewUserEntity) -> Result<Uuid>;
        async fn find_by_id(&self, id: Uuid) -> Result<UserEntity>;
        async fn find_by_username(&self, username: String) -> Result<UserEntity>;
        async fn find_by_email(&self, email: String) -> Result<UserEntity>;
        async fn update_last_login(&self, id: Uuid) -> Result<()>;
        async fn increment_failed_login(&self, id: Uuid) -> Result<()>;
        async fn reset_failed_login(&self, id: Uuid) -> Result<()>;
        async fn lock_account(&self, id: Uuid) -> Result<()>;
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
        async fn admin_update(&self, user_id: Uuid, params: AdminUpdateUserParams) -> Result<()>;
        async fn find_all(&self) -> Result<Vec<UserEntity>>;
        async fn find_paginated(&self, limit: i64, offset: i64) -> Result<Vec<UserEntity>>;
        async fn count(&self) -> Result<i64>;
        async fn delete(&self, id: Uuid) -> Result<()>;
        async fn find_by_verification_token(&self, token: String) -> Result<Option<UserEntity>>;
        async fn mark_as_verified(&self, id: Uuid) -> Result<()>;
        async fn find_by_password_reset_token(&self, token: String) -> Result<Option<UserEntity>>;
        async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()>;
        async fn update_2fa_status(
            &self,
            id: Uuid,
            secret: Option<String>,
            enabled: bool,
            backup_codes: Vec<Option<String>>,
        ) -> Result<()>;
    }
}

// Mock RoleRepository
mock! {
    pub RoleRepo {}

    #[async_trait]
    impl RoleRepository for RoleRepo {
        async fn create(&self, new_role: NewRoleEntity) -> Result<Uuid>;
        async fn find_by_id(&self, id: Uuid) -> Result<RoleEntity>;
        async fn find_by_name(&self, name: String) -> Result<RoleEntity>;
        async fn find_all(&self) -> Result<Vec<RoleEntity>>;
        async fn update(
            &self,
            id: Uuid,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<()>;
        async fn delete(&self, id: Uuid) -> Result<()>;
        async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
        async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
        async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<PermissionEntity>>;
    }
}

// Mock PermissionRepository
mock! {
    pub PermissionRepo {}

    #[async_trait]
    impl PermissionRepository for PermissionRepo {
        async fn create(&self, new_permission: NewPermissionEntity) -> Result<Uuid>;
        async fn find_by_id(&self, id: Uuid) -> Result<PermissionEntity>;
        async fn find_by_name(&self, name: String) -> Result<PermissionEntity>;
        async fn find_all(&self) -> Result<Vec<PermissionEntity>>;
        async fn update(
            &self,
            id: Uuid,
            name: Option<String>,
            description: Option<String>,
        ) -> Result<()>;
        async fn delete(&self, id: Uuid) -> Result<()>;
    }
}

// Mock BlacklistRepository
mock! {
    pub BlacklistRepo {}

    #[async_trait]
    impl BlacklistRepository for BlacklistRepo {
        async fn add(&self, jti: Uuid, expires_at: DateTime<Utc>) -> Result<()>;
        async fn is_blacklisted(&self, jti: Uuid) -> Result<bool>;
        async fn cleanup_expired(&self) -> Result<usize>;
    }
}

// Mock UserSocialAccountRepository
mock! {
    pub UserSocialAccountRepo {}

    #[async_trait]
    impl UserSocialAccountRepository for UserSocialAccountRepo {
        async fn create(&self, new_account: NewUserSocialAccountEntity) -> Result<UserSocialAccountEntity>;
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
            update: crate::domain::entities::user_social_account::UpdateUserSocialAccountEntity,
        ) -> Result<UserSocialAccountEntity>;
        async fn delete(&self, id: Uuid) -> Result<()>;
    }
}
