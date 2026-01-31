use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub actor_id: mongodb::bson::Uuid,
    pub event_type: String,
    pub target_id: Option<mongodb::bson::Uuid>,
    pub resource: String,
    pub action: String,
    pub metadata: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

pub struct NewAuditLogEntity {
    pub actor_id: Uuid,
    pub event_type: String,
    pub target_id: Option<Uuid>,
    pub resource: String,
    pub action: String,
    pub metadata: serde_json::Value,
}
