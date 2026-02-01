use anyhow::{Result, anyhow};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::user::{NewUserEntity, UserEntity};
use crate::domain::entities::user_social_account::{
    NewUserSocialAccountEntity, UpdateUserSocialAccountEntity,
};
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::repositories::user_social_account_repository::UserSocialAccountRepository;
use crate::services::jwt_service::JwtService;
use crate::services::oauth2_service::OAuth2Service;

pub struct OAuth2UseCases {
    user_repo: Arc<dyn UserRepository>,
    social_account_repo: Arc<dyn UserSocialAccountRepository>,
    oauth2_service: Arc<OAuth2Service>,
    jwt_service: Arc<JwtService>,
}

impl OAuth2UseCases {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        social_account_repo: Arc<dyn UserSocialAccountRepository>,
        oauth2_service: Arc<OAuth2Service>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            user_repo,
            social_account_repo,
            oauth2_service,
            jwt_service,
        }
    }

    /// Generate OAuth2 authorization URL for Google
    pub fn get_google_auth_url(&self) -> (String, String, String) {
        self.oauth2_service.get_google_auth_url()
    }

    /// Generate OAuth2 authorization URL for GitHub
    pub fn get_github_auth_url(&self) -> (String, String) {
        self.oauth2_service.get_github_auth_url()
    }

    /// Handle Google OAuth2 callback with account linking
    pub async fn handle_google_callback(
        &self,
        code: String,
        state: String,
        expected_state: String,
        pkce_verifier: String,
    ) -> Result<(String, UserEntity)> {
        // Validate CSRF state
        if state != expected_state {
            return Err(anyhow!("Invalid state token - CSRF protection triggered"));
        }

        // Exchange code for tokens
        let (access_token, refresh_token, expires_in) = self
            .oauth2_service
            .exchange_google_code(code, pkce_verifier)
            .await?;

        // Get user info from Google
        let google_user = self
            .oauth2_service
            .get_google_user_info(&access_token)
            .await?;

        // Check if social account already exists
        if let Some(existing_account) = self
            .social_account_repo
            .find_by_provider_user("google", &google_user.sub)
            .await?
        {
            // Update tokens and return existing user
            let expires_at =
                expires_in.map(|secs| Utc::now().naive_utc() + chrono::Duration::seconds(secs));

            self.social_account_repo
                .update(
                    existing_account.id,
                    UpdateUserSocialAccountEntity {
                        provider_email: Some(google_user.email.clone()),
                        access_token: Some(access_token),
                        refresh_token,
                        expires_at: Some(expires_at),
                        updated_at: Utc::now().naive_utc(),
                    },
                )
                .await?;

            let user = self.user_repo.find_by_id(existing_account.user_id).await?;
            let token = self
                .jwt_service
                .generate_access_token(user.id, vec![], vec![])?;
            return Ok((token, user));
        }

        // Check if user exists with this email
        let user = match self
            .user_repo
            .find_by_email(google_user.email.clone())
            .await
        {
            Ok(existing_user) => {
                // SECURITY: Only auto-link if the existing user's email is verified
                if !existing_user.is_verified.unwrap_or(false) {
                    return Err(anyhow!(
                        "An account with this email exists but is not verified. Please verify your email first or use password login."
                    ));
                }
                existing_user
            }
            Err(_) => {
                // Create new user
                let new_user = NewUserEntity {
                    username: format!("google_{}", &google_user.sub[..8]),
                    email: google_user.email.clone(),
                    password_hash: String::new(), // No password for OAuth users
                    display_name: google_user.name.clone(),
                    avatar_image_url: google_user.picture.clone(),
                    verification_token: None,
                    verification_token_expires_at: None,
                };

                let user_id = self.user_repo.create(new_user).await?;
                self.user_repo.find_by_id(user_id).await?
            }
        };

        // Create social account link
        let expires_at =
            expires_in.map(|secs| Utc::now().naive_utc() + chrono::Duration::seconds(secs));

        self.social_account_repo
            .create(NewUserSocialAccountEntity {
                id: Uuid::new_v4(),
                user_id: user.id,
                provider: "google".to_string(),
                provider_user_id: google_user.sub,
                provider_email: Some(google_user.email.clone()),
                access_token: Some(access_token),
                refresh_token,
                expires_at,
            })
            .await?;

        // Generate JWT
        let token = self
            .jwt_service
            .generate_access_token(user.id, vec![], vec![])?;
        Ok((token, user))
    }

    /// Handle GitHub OAuth2 callback with account linking
    pub async fn handle_github_callback(
        &self,
        code: String,
        state: String,
        expected_state: String,
    ) -> Result<(String, UserEntity)> {
        // Validate CSRF state
        if state != expected_state {
            return Err(anyhow!("Invalid state token - CSRF protection triggered"));
        }

        // Exchange code for tokens
        let (access_token, refresh_token, expires_in) =
            self.oauth2_service.exchange_github_code(code).await?;

        // Get user info from GitHub
        let github_user = self
            .oauth2_service
            .get_github_user_info(&access_token)
            .await?;

        let github_email = github_user.email.clone().ok_or_else(|| {
            anyhow!("GitHub did not provide an email address. Please make your email public in GitHub settings.")
        })?;

        // Check if social account already exists
        if let Some(existing_account) = self
            .social_account_repo
            .find_by_provider_user("github", &github_user.id.to_string())
            .await?
        {
            // Update tokens and return existing user
            let expires_at =
                expires_in.map(|secs| Utc::now().naive_utc() + chrono::Duration::seconds(secs));

            self.social_account_repo
                .update(
                    existing_account.id,
                    UpdateUserSocialAccountEntity {
                        provider_email: Some(github_email.clone()),
                        access_token: Some(access_token),
                        refresh_token,
                        expires_at: Some(expires_at),
                        updated_at: Utc::now().naive_utc(),
                    },
                )
                .await?;

            let user = self.user_repo.find_by_id(existing_account.user_id).await?;
            let token = self
                .jwt_service
                .generate_access_token(user.id, vec![], vec![])?;
            return Ok((token, user));
        }

        // Check if user exists with this email
        let user = match self.user_repo.find_by_email(github_email.clone()).await {
            Ok(existing_user) => {
                // SECURITY: Only auto-link if the existing user's email is verified
                if !existing_user.is_verified.unwrap_or(false) {
                    return Err(anyhow!(
                        "An account with this email exists but is not verified. Please verify your email first or use password login."
                    ));
                }
                existing_user
            }
            Err(_) => {
                // Create new user
                let new_user = NewUserEntity {
                    username: github_user.login.clone(),
                    email: github_email.clone(),
                    password_hash: String::new(), // No password for OAuth users
                    display_name: github_user
                        .name
                        .unwrap_or_else(|| github_user.login.clone()),
                    avatar_image_url: github_user.avatar_url,
                    verification_token: None,
                    verification_token_expires_at: None,
                };

                let user_id = self.user_repo.create(new_user).await?;
                self.user_repo.find_by_id(user_id).await?
            }
        };

        // Create social account link
        let expires_at =
            expires_in.map(|secs| Utc::now().naive_utc() + chrono::Duration::seconds(secs));

        self.social_account_repo
            .create(NewUserSocialAccountEntity {
                id: Uuid::new_v4(),
                user_id: user.id,
                provider: "github".to_string(),
                provider_user_id: github_user.id.to_string(),
                provider_email: Some(github_email.clone()),
                access_token: Some(access_token),
                refresh_token,
                expires_at,
            })
            .await?;

        // Generate JWT
        let token = self
            .jwt_service
            .generate_access_token(user.id, vec![], vec![])?;
        Ok((token, user))
    }
}
