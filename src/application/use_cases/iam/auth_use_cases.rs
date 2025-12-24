use crate::domain::entities::iam::user::{User, UserCredentials};
use crate::domain::services::iam::AuthenticationService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub user: User,
}

#[async_trait]
pub trait AuthUseCases: Send + Sync {
    async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Box<dyn Error>>;
    async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, Box<dyn Error>>;
    async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, Box<dyn Error>>;
    async fn logout(&self, token: String) -> Result<(), Box<dyn Error>>;
    async fn validate_token(&self, token: String) -> Result<Option<User>, Box<dyn Error>>;
}

pub struct AuthUseCasesImpl<T: AuthenticationService> {
    auth_service: T,
}

impl<T: AuthenticationService> AuthUseCasesImpl<T> {
    pub fn new(auth_service: T) -> Self {
        Self { auth_service }
    }
}

#[async_trait]
impl<T: AuthenticationService> AuthUseCases for AuthUseCasesImpl<T> {
    async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
        let credentials = UserCredentials {
            username: request.username,
            password: request.password,
        };

        let user = self
            .auth_service
            .authenticate(credentials)
            .await?
            .ok_or("Invalid credentials")?;

        let token = self.auth_service.generate_token(&user).await?;

        Ok(LoginResponse { token, user })
    }

    async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, Box<dyn Error>> {
        // Hash the password
        let password_hash = self.auth_service.hash_password(&request.password).await?;

        // Create user
        let user = self
            .auth_service
            .create_user(crate::domain::entities::iam::user::NewUser {
                username: request.username,
                email: request.email,
                password_hash,
                first_name: request.first_name,
                last_name: request.last_name,
            })
            .await?;

        // Generate token
        let token = self.auth_service.generate_token(&user).await?;

        Ok(RegisterResponse { user, token })
    }

    async fn refresh_token(
        &self,
        request: RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, Box<dyn Error>> {
        let user = self
            .auth_service
            .verify_token(&request.token)
            .await?
            .ok_or("Invalid token")?;

        let new_token = self.auth_service.generate_token(&user).await?;

        Ok(RefreshTokenResponse {
            token: new_token,
            user,
        })
    }

    async fn logout(&self, token: String) -> Result<(), Box<dyn Error>> {
        // In a real implementation, you might want to blacklist the token
        // For now, we'll just validate the token to ensure it's valid
        self.auth_service.verify_token(&token).await?;
        Ok(())
    }

    async fn validate_token(&self, token: String) -> Result<Option<User>, Box<dyn Error>> {
        self.auth_service.verify_token(&token).await
    }
}
