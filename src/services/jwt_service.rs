use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String, // Subject (user ID)
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct JwtService {
    access_secret: String,
    refresh_secret: String,
}

impl JwtService {
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_secret,
            refresh_secret,
        }
    }

    /// Generate access token (15 minutes expiry)
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(15);

        let claims = TokenClaims {
            sub: user_id.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            roles,
            permissions,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Generate refresh token (7 days expiry)
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String> {
        let now = Utc::now();
        let expiry = now + Duration::days(7);

        let claims = TokenClaims {
            sub: user_id.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            roles: vec![],
            permissions: vec![],
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.refresh_secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Generate both access and refresh tokens
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<TokenPair> {
        let access_token = self.generate_access_token(user_id, roles, permissions)?;
        let refresh_token = self.generate_refresh_token(user_id)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// Validate and decode access token
    pub fn validate_access_token(&self, token: &str) -> Result<TokenClaims> {
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.access_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// Validate and decode refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<TokenClaims> {
        let token_data = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(self.refresh_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }
}
