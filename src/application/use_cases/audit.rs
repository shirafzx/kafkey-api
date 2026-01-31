use crate::domain::entities::audit_log::NewAuditLogEntity;
use crate::domain::repositories::audit_repository::AuditRepository;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuditUseCases<R: AuditRepository> {
    audit_repository: Arc<R>,
}

impl<R: AuditRepository> AuditUseCases<R> {
    pub fn new(audit_repository: Arc<R>) -> Self {
        Self { audit_repository }
    }

    pub async fn log(
        &self,
        actor_id: Uuid,
        event_type: &str,
        target_id: Option<Uuid>,
        resource: &str,
        action: &str,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let new_audit = NewAuditLogEntity {
            actor_id,
            event_type: event_type.to_string(),
            target_id,
            resource: resource.to_string(),
            action: action.to_string(),
            metadata,
        };

        self.audit_repository.create(new_audit).await
    }
}
