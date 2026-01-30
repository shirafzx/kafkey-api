use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait BlacklistRepository {
    async fn add(&self, jti: Uuid, expires_at: DateTime<Utc>) -> Result<()>;
    async fn is_blacklisted(&self, jti: Uuid) -> Result<bool>;
    async fn cleanup_expired(&self) -> Result<usize>;
}
