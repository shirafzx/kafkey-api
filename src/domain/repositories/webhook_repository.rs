use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::webhook::{
    NewWebhookDeliveryEntity, NewWebhookEntity, UpdateWebhookEntity, WebhookDeliveryEntity,
    WebhookEntity,
};

#[async_trait]
pub trait WebhookRepository: Send + Sync {
    // Webhook management
    async fn create(&self, new_webhook: NewWebhookEntity) -> Result<Uuid>;
    async fn find_by_id(&self, id: Uuid) -> Result<WebhookEntity>;
    async fn find_by_id_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<WebhookEntity>;
    async fn find_all_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WebhookEntity>>;
    async fn update_scoped(
        &self,
        id: Uuid,
        tenant_id: Uuid,
        update_data: UpdateWebhookEntity,
    ) -> Result<()>;
    async fn delete_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<()>;

    // Deliveries
    async fn log_delivery(&self, delivery: NewWebhookDeliveryEntity) -> Result<Uuid>;
    async fn find_deliveries_by_webhook_id(
        &self,
        webhook_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WebhookDeliveryEntity>>;

    // For event processing - find webhooks interested in an event
    async fn find_active_webhooks_by_event(
        &self,
        tenant_id: Uuid,
        event_type: &str,
    ) -> Result<Vec<WebhookEntity>>;
}
