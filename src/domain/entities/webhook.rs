use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::{webhook_deliveries, webhooks};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = webhooks)]
pub struct WebhookEntity {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub url: String,
    pub events: Vec<Option<String>>,
    #[serde(skip_serializing)]
    pub secret: String,
    pub is_active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = webhooks)]
pub struct NewWebhookEntity {
    pub tenant_id: Uuid,
    pub url: String,
    pub events: Vec<String>,
    pub secret: String,
    pub is_active: bool,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations,
)]
#[diesel(belongs_to(WebhookEntity, foreign_key = webhook_id))]
#[diesel(table_name = webhook_deliveries)]
pub struct WebhookDeliveryEntity {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = webhook_deliveries)]
pub struct NewWebhookDeliveryEntity {
    pub webhook_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = webhooks)]
pub struct UpdateWebhookEntity {
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}
