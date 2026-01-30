use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::repositories::{
        blacklist_repository::BlacklistRepository, role_repository::RoleRepository,
        user_repository::UserRepository,
    },
    services::{jwt_service::JwtService, password_service::PasswordService},
};

pub struct AuthUseCases<T, R, B>
where
    T: UserRepository + Send + Sync,
    R: RoleRepository + Send + Sync,
    B: BlacklistRepository + Send + Sync,
{
    user_repository: Arc<T>,
    role_repository: Arc<R>,
    blacklist_repository: Arc<B>,
    jwt_service: Arc<JwtService>,
}

impl<T, R, B> AuthUseCases<T, R, B>
where
    T: UserRepository + Send + Sync,
    R: RoleRepository + Send + Sync,
    B: BlacklistRepository + Send + Sync,
{
    pub fn new(
        user_repository: Arc<T>,
        role_repository: Arc<R>,
        blacklist_repository: Arc<B>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            user_repository,
            role_repository,
            blacklist_repository,
            jwt_service,
        }
    }

    /// Register a new user
    pub async fn register(
        &self,
        username: String,
        email: String,
        display_name: String,
        password: String,
        avatar_image_url: Option<String>,
    ) -> Result<Uuid> {
        // Hash password
        let password_hash = PasswordService::hash_password(&password)?;

        // Create user entity
        let new_user = crate::domain::entities::user::NewUserEntity {
            username,
            email,
            display_name,
            avatar_image_url,
            password_hash,
        };

        // Create user
        let user_id = self.user_repository.create(new_user).await?;

        // Assign default "user" role
        match self.role_repository.find_by_name("user".to_string()).await {
            Ok(role) => {
                if let Err(e) = self.user_repository.assign_role(user_id, role.id).await {
                    tracing::warn!("Failed to assign default role for user {}: {}", user_id, e);
                }
            }
            Err(e) => tracing::warn!("Failed to find default role during registration: {}", e),
        }

        Ok(user_id)
    }

    /// Authenticate user and generate tokens
    pub async fn login(
        &self,
        email_or_username: String,
        password: String,
    ) -> Result<(Uuid, String, String)> {
        // Try to find user by email or username
        let user = match self
            .user_repository
            .find_by_email(email_or_username.clone())
            .await
        {
            Ok(user) => user,
            Err(_) => {
                self.user_repository
                    .find_by_username(email_or_username)
                    .await?
            }
        };

        // Verify password
        let is_valid = PasswordService::verify_password(&password, &user.password_hash)?;
        if !is_valid {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Check if user is active
        if !user.is_active.unwrap_or(true) {
            return Err(anyhow::anyhow!("User account is deactivated"));
        }

        // Update last login
        self.user_repository.update_last_login(user.id).await?;

        // Get user roles and permissions
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let permissions = self.user_repository.get_user_permissions(user.id).await?;

        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permission_names: Vec<String> = permissions.iter().map(|p| p.name.clone()).collect();

        // Generate tokens
        let token_pair =
            self.jwt_service
                .generate_token_pair(user.id, role_names, permission_names)?;

        Ok((user.id, token_pair.access_token, token_pair.refresh_token))
    }

    /// Logout user by blacklisting their tokens
    pub async fn logout(&self, access_token: String, refresh_token: Option<String>) -> Result<()> {
        use chrono::DateTime;

        // Blacklist access token
        if let Ok(claims) = self.jwt_service.validate_access_token(&access_token) {
            let jti = Uuid::parse_str(&claims.jti)?;
            let exp = DateTime::from_timestamp(claims.exp, 0)
                .ok_or_else(|| anyhow::anyhow!("Invalid expiration timestamp"))?;
            self.blacklist_repository.add(jti, exp).await?;
        }

        // Blacklist refresh token if provided
        if let Some(rt) = refresh_token {
            if let Ok(claims) = self.jwt_service.validate_refresh_token(&rt) {
                let jti = Uuid::parse_str(&claims.jti)?;
                let exp = DateTime::from_timestamp(claims.exp, 0)
                    .ok_or_else(|| anyhow::anyhow!("Invalid expiration timestamp"))?;
                self.blacklist_repository.add(jti, exp).await?;
            }
        }

        Ok(())
    }

    /// Refresh access token using refresh token
    pub async fn refresh_token(&self, refresh_token: String) -> Result<String> {
        // Validate refresh token
        let claims = self.jwt_service.validate_refresh_token(&refresh_token)?;

        // Check if blacklisted
        let jti = Uuid::parse_str(&claims.jti)?;
        if self.blacklist_repository.is_blacklisted(jti).await? {
            return Err(anyhow::anyhow!("Token is blacklisted"));
        }

        let user_id = Uuid::parse_str(&claims.sub)?;

        // Get user to ensure they still exist and are active
        let user = self.user_repository.find_by_id(user_id).await?;
        if !user.is_active.unwrap_or(true) {
            return Err(anyhow::anyhow!("User account is deactivated"));
        }

        // Get updated roles and permissions
        let roles = self.user_repository.get_user_roles(user_id).await?;
        let permissions = self.user_repository.get_user_permissions(user_id).await?;

        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permission_names: Vec<String> = permissions.iter().map(|p| p.name.clone()).collect();

        // Generate new access token
        let access_token =
            self.jwt_service
                .generate_access_token(user_id, role_names, permission_names)?;

        Ok(access_token)
    }
}
