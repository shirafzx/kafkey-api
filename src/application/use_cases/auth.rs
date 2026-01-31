use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::dtos::LoginResponse,
    domain::entities::user::AdminUpdateUserParams,
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

        // Generate verification token (32 character hex)
        let verification_token = Some(Uuid::new_v4().to_string().replace("-", ""));
        let verification_token_expires_at = Some(chrono::Utc::now() + chrono::Duration::hours(24));

        // Create user entity
        let new_user = crate::domain::entities::user::NewUserEntity {
            username: username.clone(),
            email: email.clone(),
            display_name,
            avatar_image_url,
            password_hash,
            verification_token,
            verification_token_expires_at,
        };

        // Create user
        let user_id = self.user_repository.create(new_user).await?;

        tracing::info!(
            event = "AUTH_REGISTER_SUCCESS",
            user_id = %user_id,
            username = %username,
            email = %email,
            "User registered successfully"
        );

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

    /// Verify user email using a token
    pub async fn verify_email(&self, token: String) -> Result<()> {
        let user = self
            .user_repository
            .find_by_verification_token(token)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid or expired verification token"))?;

        // Check expiration
        if let Some(expires_at) = user.verification_token_expires_at
            && chrono::Utc::now() > expires_at
        {
            return Err(anyhow::anyhow!("Verification token has expired"));
        }

        // Mark as verified
        self.user_repository.mark_as_verified(user.id).await?;

        tracing::info!(
            event = "AUTH_EMAIL_VERIFIED",
            user_id = %user.id,
            "User email verified successfully"
        );

        Ok(())
    }

    /// Resend verification email
    pub async fn resend_verification_email(&self, email_or_username: String) -> Result<()> {
        // Find user
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

        if user.is_verified.unwrap_or(false) {
            return Err(anyhow::anyhow!("Email is already verified"));
        }

        // Generate new token
        let verification_token = Uuid::new_v4().to_string().replace("-", "");
        let verification_token_expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

        // Update user
        self.user_repository
            .admin_update(
                user.id,
                AdminUpdateUserParams {
                    verification_token: Some(Some(verification_token)),
                    verification_token_expires_at: Some(Some(verification_token_expires_at)),
                    ..Default::default()
                },
            )
            .await?;

        Ok(())
    }

    /// Authenticate user and generate tokens
    pub async fn login(
        &self,
        email_or_username: String,
        password: String,
    ) -> Result<LoginResponse> {
        const MAX_FAILED_ATTEMPTS: i32 = 5;
        const LOCKOUT_DURATION_MINUTES: i64 = 30;

        // Try to find user by email or username
        let user = match self
            .user_repository
            .find_by_email(email_or_username.clone())
            .await
        {
            Ok(user) => user,
            Err(_) => {
                self.user_repository
                    .find_by_username(email_or_username.clone())
                    .await?
            }
        };

        // Check if account is locked
        if let Some(locked_at) = user.locked_at {
            let now = chrono::Utc::now();
            let lockout_duration = chrono::Duration::minutes(LOCKOUT_DURATION_MINUTES);
            if now < locked_at + lockout_duration {
                let remaining = (locked_at + lockout_duration) - now;
                let minutes = remaining.num_minutes() + 1;

                tracing::warn!(
                    event = "AUTH_LOGIN_LOCKED",
                    user_id = %user.id,
                    remaining_minutes = minutes,
                    "Login attempted on locked account"
                );

                return Err(anyhow::anyhow!(
                    "Account is locked. Please try again in {} minutes.",
                    minutes
                ));
            } else {
                // Lockout expired, reset for this attempt
                self.user_repository.reset_failed_login(user.id).await?;
            }
        }

        // Verify password
        let is_valid = PasswordService::verify_password(&password, &user.password_hash)?;
        if !is_valid {
            // Increment failed attempts
            self.user_repository.increment_failed_login(user.id).await?;

            // Check if we should lock now
            if user.failed_login_attempts + 1 >= MAX_FAILED_ATTEMPTS {
                self.user_repository.lock_account(user.id).await?;

                tracing::warn!(
                    event = "AUTH_ACCOUNT_LOCKOUT",
                    user_id = %user.id,
                    email = %user.email,
                    "Account locked due to multiple failed login attempts"
                );

                return Err(anyhow::anyhow!(
                    "Too many failed attempts. Account locked for {} minutes.",
                    LOCKOUT_DURATION_MINUTES
                ));
            }

            tracing::warn!(
                event = "AUTH_LOGIN_FAILED",
                email_or_username = %email_or_username,
                reason = "Invalid credentials",
                "Login failed"
            );

            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        // Check if user is active
        if !user.is_active.unwrap_or(true) {
            return Err(anyhow::anyhow!("User account is deactivated"));
        }

        // Successful login: Reset failed attempts and update last login
        self.user_repository.reset_failed_login(user.id).await?;
        self.user_repository.update_last_login(user.id).await?;

        // Get user roles and permissions
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let permissions = self.user_repository.get_user_permissions(user.id).await?;

        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permission_names: Vec<String> = permissions.iter().map(|p| p.name.clone()).collect();

        // Check if 2FA is enabled
        if user.two_factor_enabled {
            tracing::info!(
                event = "AUTH_MFA_CHALLENGE",
                user_id = %user.id,
                "MFA challenge required for user login"
            );

            return Ok(LoginResponse::RequiresMfa {
                user_id: user.id.to_string(),
            });
        }

        // Generate tokens
        let token_pair =
            self.jwt_service
                .generate_token_pair(user.id, role_names, permission_names)?;

        let res = Ok(LoginResponse::Success(
            crate::application::dtos::AuthResponse {
                user_id: user.id.to_string(),
                access_token: token_pair.access_token,
                refresh_token: token_pair.refresh_token,
            },
        ));

        tracing::info!(
            event = "AUTH_LOGIN_SUCCESS",
            user_id = %user.id,
            "User logged in successfully"
        );

        res
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
        if let Some(rt) = refresh_token
            && let Ok(claims) = self.jwt_service.validate_refresh_token(&rt)
        {
            let jti = Uuid::parse_str(&claims.jti)?;
            let exp = DateTime::from_timestamp(claims.exp, 0)
                .ok_or_else(|| anyhow::anyhow!("Invalid expiration timestamp"))?;
            self.blacklist_repository.add(jti, exp).await?;
        }

        tracing::info!(event = "AUTH_LOGOUT", "User logged out successfully");

        Ok(())
    }

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

        tracing::debug!(
            event = "AUTH_TOKEN_REFRESH",
            user_id = %user_id,
            "Access token refreshed"
        );

        Ok(access_token)
    }

    /// Initiate password reset
    pub async fn forgot_password(&self, email: String) -> Result<()> {
        let user = self.user_repository.find_by_email(email).await?;

        // Generate reset token (32 character hex)
        let token = Uuid::new_v4().to_string().replace("-", "");
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

        // Store token
        self.user_repository
            .admin_update(
                user.id,
                AdminUpdateUserParams {
                    password_reset_token: Some(Some(token)),
                    password_reset_expires_at: Some(Some(expires_at)),
                    ..Default::default()
                },
            )
            .await?;

        // In a real app, send email here
        tracing::info!(
            event = "AUTH_PASSWORD_RESET_REQUESTED",
            user_id = %user.id,
            "Password reset token generated"
        );

        Ok(())
    }

    /// Reset password using token
    pub async fn reset_password(&self, token: String, new_password: String) -> Result<()> {
        let user = self
            .user_repository
            .find_by_password_reset_token(token)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Invalid or expired reset token"))?;

        // Check expiration
        if let Some(expires_at) = user.password_reset_expires_at
            && chrono::Utc::now() > expires_at
        {
            return Err(anyhow::anyhow!("Reset token has expired"));
        }

        // Hash new password
        let password_hash = PasswordService::hash_password(&new_password)?;

        // Update password (methods clear token)
        self.user_repository
            .update_password(user.id, password_hash)
            .await?;

        tracing::info!(
            event = "AUTH_PASSWORD_RESET_SUCCESS",
            user_id = %user.id,
            "Password reset successfully completed"
        );

        Ok(())
    }

    /// Generate 2FA setup details (secret and provisioning URL)
    pub async fn generate_2fa_setup(&self, user_id: Uuid) -> Result<(String, String)> {
        let user = self.user_repository.find_by_id(user_id).await?;

        if user.two_factor_enabled {
            return Err(anyhow::anyhow!("2FA is already enabled"));
        }

        let secret = crate::services::totp_service::TotpService::generate_secret();
        let qr_url = crate::services::totp_service::TotpService::generate_qr_code_url(
            &secret,
            &user.email,
            "KafkeyAPI", // Could be configurable
        )?;

        Ok((secret, qr_url))
    }

    /// Confirm 2FA setup and enable it
    pub async fn confirm_2fa_setup(
        &self,
        user_id: Uuid,
        secret: String,
        code: String,
    ) -> Result<Vec<String>> {
        // Verify code
        if !crate::services::totp_service::TotpService::verify_code(&secret, &code) {
            return Err(anyhow::anyhow!("Invalid verification code"));
        }

        // Generate backup codes
        let backup_codes = self.generate_backup_codes();
        let backup_codes_opt: Vec<Option<String>> =
            backup_codes.iter().map(|c| Some(c.clone())).collect();

        // Update user status
        self.user_repository
            .update_2fa_status(user_id, Some(secret), true, backup_codes_opt)
            .await?;

        tracing::info!(
            event = "AUTH_2FA_ENABLED",
            user_id = %user_id,
            "Two-factor authentication enabled"
        );

        Ok(backup_codes)
    }

    /// Disable 2FA
    pub async fn disable_2fa(&self, user_id: Uuid, code: String) -> Result<()> {
        let user = self.user_repository.find_by_id(user_id).await?;

        if !user.two_factor_enabled {
            return Err(anyhow::anyhow!("2FA is not enabled"));
        }

        let secret = user
            .two_factor_secret
            .ok_or_else(|| anyhow::anyhow!("Missing 2FA secret"))?;

        // Verify code
        if !crate::services::totp_service::TotpService::verify_code(&secret, &code) {
            return Err(anyhow::anyhow!("Invalid verification code"));
        }

        // Disable 2FA
        self.user_repository
            .update_2fa_status(user_id, None, false, vec![])
            .await?;

        tracing::info!(
            event = "AUTH_2FA_DISABLED",
            user_id = %user_id,
            "Two-factor authentication disabled"
        );

        Ok(())
    }

    /// Verify 2FA login code
    pub async fn verify_2fa_login(&self, user_id: Uuid, code: String) -> Result<(String, String)> {
        let user = self.user_repository.find_by_id(user_id).await?;

        if !user.two_factor_enabled {
            return Err(anyhow::anyhow!("2FA is not enabled for this user"));
        }

        let secret = user
            .two_factor_secret
            .ok_or_else(|| anyhow::anyhow!("Missing 2FA secret"))?;

        // Verify code
        let is_valid = crate::services::totp_service::TotpService::verify_code(&secret, &code);

        // Check backup codes if TOTP fails
        if !is_valid {
            // Check backup codes
            let backup_codes = user.two_factor_backup_codes.unwrap_or_default();
            let mut found = false;
            let mut new_codes = Vec::new();

            for c in backup_codes.into_iter().flatten() {
                if c == code && !found {
                    found = true;
                    continue; // Consume this code
                }
                new_codes.push(Some(c));
            }

            if !found {
                return Err(anyhow::anyhow!("Invalid 2FA code"));
            }

            // Update user with consumed backup code
            self.user_repository
                .update_2fa_status(user.id, Some(secret), true, new_codes)
                .await?;
        }

        // Get user roles and permissions
        let roles = self.user_repository.get_user_roles(user.id).await?;
        let permissions = self.user_repository.get_user_permissions(user.id).await?;

        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        let permission_names: Vec<String> = permissions.iter().map(|p| p.name.clone()).collect();

        // Generate tokens
        let token_pair =
            self.jwt_service
                .generate_token_pair(user.id, role_names, permission_names)?;

        tracing::info!(
            event = "AUTH_2FA_VERIFY_SUCCESS",
            user_id = %user.id,
            "MFA verification successful"
        );

        Ok((token_pair.access_token, token_pair.refresh_token))
    }

    fn generate_backup_codes(&self) -> Vec<String> {
        use rand::distributions::Alphanumeric;
        use rand::{Rng, thread_rng};

        (0..10)
            .map(|_| {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect()
            })
            .collect()
    }
}
