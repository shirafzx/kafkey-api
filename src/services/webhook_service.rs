use anyhow::{Context, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;
use std::sync::Arc;
use tokio::time::{Duration, sleep};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::{
    entities::webhook::{NewWebhookDeliveryEntity, WebhookEntity},
    repositories::webhook_repository::WebhookRepository,
};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 1000;

#[derive(Clone)]
pub struct WebhookService {
    repository: Arc<dyn WebhookRepository>,
    http_client: Client,
}

impl WebhookService {
    pub fn new(repository: Arc<dyn WebhookRepository>) -> Self {
        Self {
            repository,
            http_client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn trigger_event(
        &self,
        tenant_id: Uuid,
        event_type: &str,
        payload: Value,
    ) -> Result<()> {
        // Find active webhooks for this tenant and event
        let webhooks = self
            .repository
            .find_active_webhooks_by_event(tenant_id, event_type)
            .await?;

        if webhooks.is_empty() {
            return Ok(());
        }

        info!(
            "Triggering {} webhooks for tenant {} event {}",
            webhooks.len(),
            tenant_id,
            event_type
        );

        // Process webhooks concurrently
        for webhook in webhooks {
            let service = self.clone();
            let payload = payload.clone();
            let event_type = event_type.to_string();

            tokio::spawn(async move {
                if let Err(e) = service.deliver_webhook(webhook, event_type, payload).await {
                    error!("Failed to deliver webhook: {:?}", e);
                }
            });
        }

        Ok(())
    }

    async fn deliver_webhook(
        &self,
        webhook: WebhookEntity,
        event_type: String,
        payload: Value,
    ) -> Result<()> {
        // Prepare signature
        let payload_string = serde_json::to_string(&payload)?;
        let signature = self.sign_payload(&webhook.secret, &payload_string)?;

        let mut attempt = 0;
        let mut success = false;
        let mut last_error = None;
        let mut response_status = None;
        let mut response_body = None;

        while attempt < MAX_RETRIES {
            attempt += 1;

            let request = self
                .http_client
                .post(&webhook.url)
                .header("Content-Type", "application/json")
                .header("X-Webhook-Event", &event_type)
                .header("X-Webhook-Signature", &signature)
                .body(payload_string.clone());

            match request.send().await {
                Ok(response) => {
                    response_status = Some(response.status().as_u16() as i32);
                    let status_success = response.status().is_success();

                    if status_success {
                        success = true;
                        response_body = Some("Success".to_string());
                        break;
                    } else {
                        let body = response.text().await.unwrap_or_default();
                        response_body = Some(body.chars().take(1000).collect()); // Truncate body
                        warn!(
                            "Webhook delivery failed (attempt {}/{}): Status {}",
                            attempt,
                            MAX_RETRIES,
                            response_status.unwrap()
                        );
                    }
                }
                Err(e) => {
                    last_error = Some(e.to_string());
                    warn!(
                        "Webhook delivery network error (attempt {}/{}): {}",
                        attempt, MAX_RETRIES, e
                    );
                }
            }

            if attempt < MAX_RETRIES {
                let backoff = Duration::from_millis(RETRY_DELAY_MS * 2_u64.pow(attempt - 1));
                sleep(backoff).await;
            }
        }

        // Log delivery
        let delivery_record = NewWebhookDeliveryEntity {
            webhook_id: webhook.id,
            event_type,
            payload,
            response_status,
            response_body: response_body.or(last_error),
            delivered_at: if success { Some(Utc::now()) } else { None },
        };

        self.repository.log_delivery(delivery_record).await?;

        Ok(())
    }

    fn sign_payload(&self, secret: &str, payload: &str) -> Result<String> {
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .context("HMAC can take key of any size")?;
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }
}
