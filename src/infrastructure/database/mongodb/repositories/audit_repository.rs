use crate::domain::entities::audit_log::{AuditLogEntity, NewAuditLogEntity};
use crate::domain::repositories::audit_repository::AuditRepository;
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use mongodb::{Client, Collection};

pub struct AuditMongodb {
    collection: Collection<AuditLogEntity>,
}

impl AuditMongodb {
    pub fn new(client: &Client) -> Self {
        let db = client.database("kafkey");
        let collection = db.collection::<AuditLogEntity>("audit_logs");
        Self { collection }
    }
}

#[async_trait]
impl AuditRepository for AuditMongodb {
    async fn create(&self, new_audit: NewAuditLogEntity) -> Result<()> {
        let audit_log = AuditLogEntity {
            id: None,
            actor_id: mongodb::bson::Uuid::from_bytes(new_audit.actor_id.into_bytes()),
            event_type: new_audit.event_type,
            target_id: new_audit
                .target_id
                .map(|uuid| mongodb::bson::Uuid::from_bytes(uuid.into_bytes())),
            resource: new_audit.resource,
            action: new_audit.action,
            metadata: new_audit.metadata,
            timestamp: Utc::now(),
        };

        self.collection.insert_one(audit_log).await?;
        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<AuditLogEntity>> {
        let mut cursor = self.collection.find(mongodb::bson::doc! {}).await?;
        let mut results = Vec::new();
        while cursor.advance().await? {
            results.push(cursor.deserialize_current()?);
        }
        Ok(results)
    }
}
