use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{delete, insert_into, prelude::*};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::webhook::{
            NewWebhookDeliveryEntity, NewWebhookEntity, UpdateWebhookEntity, WebhookDeliveryEntity,
            WebhookEntity,
        },
        repositories::webhook_repository::WebhookRepository,
    },
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        schema::{webhook_deliveries, webhooks},
    },
};

pub struct WebhookPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl WebhookPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl WebhookRepository for WebhookPostgres {
    // Webhook management
    async fn create(&self, new_webhook: NewWebhookEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(webhooks::table)
            .values(new_webhook)
            .returning(webhooks::id)
            .get_result::<Uuid>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<WebhookEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = webhooks::table
            .filter(webhooks::id.eq(id))
            .select(WebhookEntity::as_select())
            .first::<WebhookEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_id_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<WebhookEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = webhooks::table
            .filter(webhooks::id.eq(id))
            .filter(webhooks::tenant_id.eq(tenant_id))
            .select(WebhookEntity::as_select())
            .first::<WebhookEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_all_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<WebhookEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = webhooks::table
            .filter(webhooks::tenant_id.eq(tenant_id))
            .select(WebhookEntity::as_select())
            .load::<WebhookEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update_scoped(
        &self,
        id: Uuid,
        tenant_id: Uuid,
        update_data: UpdateWebhookEntity,
    ) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::update(
            webhooks::table
                .filter(webhooks::id.eq(id))
                .filter(webhooks::tenant_id.eq(tenant_id)),
        )
        .set(update_data)
        .execute(&mut conn)?;

        Ok(())
    }

    async fn delete_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(
            webhooks::table
                .filter(webhooks::id.eq(id))
                .filter(webhooks::tenant_id.eq(tenant_id)),
        )
        .execute(&mut conn)?;

        Ok(())
    }

    // Deliveries
    async fn log_delivery(&self, delivery: NewWebhookDeliveryEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(webhook_deliveries::table)
            .values(delivery)
            .returning(webhook_deliveries::id)
            .get_result::<Uuid>(&mut conn)?;

        Ok(result)
    }

    async fn find_deliveries_by_webhook_id(
        &self,
        webhook_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<WebhookDeliveryEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = webhook_deliveries::table
            .filter(webhook_deliveries::webhook_id.eq(webhook_id))
            .select(WebhookDeliveryEntity::as_select())
            .order(webhook_deliveries::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<WebhookDeliveryEntity>(&mut conn)?;

        Ok(results)
    }

    async fn find_active_webhooks_by_event(
        &self,
        tenant_id: Uuid,
        event_type: &str,
    ) -> Result<Vec<WebhookEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        // SQL: SELECT * FROM webhooks WHERE tenant_id = $1 AND is_active = true AND $2 = ANY(events)
        let results = webhooks::table
            .filter(webhooks::tenant_id.eq(tenant_id))
            .filter(webhooks::is_active.eq(true))
            .filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&format!(
                "'{}' = ANY(events)",
                event_type
            )))
            .select(WebhookEntity::as_select())
            .load::<WebhookEntity>(&mut conn)?;

        Ok(results)
    }
}
