use crate::domain::entities::iam::user::{User, UserCredentials};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn authenticate(
        &self,
        credentials: UserCredentials,
    ) -> Result<Option<User>, Box<dyn Error>>;

    async fn generate_token(&self, user: &User) -> Result<String, Box<dyn Error>>;

    async fn verify_token(&self, token: &str) -> Result<Option<User>, Box<dyn Error>>;

    async fn hash_password(&self, password: &str) -> Result<String, Box<dyn Error>>;

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn Error>>;
}
