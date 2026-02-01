use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateWebhookRequest {
    #[validate(url)]
    pub url: String,
    #[validate(length(min = 1))]
    pub events: Vec<String>,
    pub secret: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateWebhookRequest {
    #[validate(url)]
    pub url: Option<String>,
    pub events: Option<Vec<String>>,
    pub secret: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub url: String,
    pub events: Vec<Option<String>>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct WebhookDeliveryResponse {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}
