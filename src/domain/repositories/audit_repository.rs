use crate::domain::entities::audit_log::{AuditLogEntity, NewAuditLogEntity};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn create(&self, audit_log: NewAuditLogEntity) -> Result<()>;
    async fn find_all(&self) -> Result<Vec<AuditLogEntity>>;
}
