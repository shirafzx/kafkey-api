use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::entities::tenant_settings::{NewTenantSettingsEntity, TenantSettingsEntity};
use crate::domain::repositories::tenant_settings_repository::TenantSettingsRepository;
use crate::infrastructure::database::postgres::schema::tenant_settings;

pub struct PostgresTenantSettingsRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresTenantSettingsRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantSettingsRepository for PostgresTenantSettingsRepository {
    async fn create(&self, tenant_id: Uuid) -> Result<Uuid> {
        let mut conn = self.pool.get()?;

        let new_settings = NewTenantSettingsEntity { tenant_id };

        let settings: TenantSettingsEntity = diesel::insert_into(tenant_settings::table)
            .values(&new_settings)
            .returning(TenantSettingsEntity::as_returning())
            .get_result(&mut conn)?;

        Ok(settings.id)
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<TenantSettingsEntity> {
        let mut conn = self.pool.get()?;

        let settings = tenant_settings::table
            .filter(tenant_settings::tenant_id.eq(tenant_id))
            .first::<TenantSettingsEntity>(&mut conn)?;

        Ok(settings)
    }

    async fn update(
        &self,
        tenant_id: Uuid,
        allow_signups: Option<bool>,
        require_email_verification: Option<bool>,
        enable_2fa: Option<bool>,
        session_duration_minutes: Option<i32>,
        allowed_oauth_providers: Option<Vec<String>>,
        webhook_url: Option<String>,
    ) -> Result<()> {
        let mut conn = self.pool.get()?;

        // Convert Vec<String> to Vec<Option<String>> for Diesel
        let providers_opt = allowed_oauth_providers
            .map(|v| v.into_iter().map(Some).collect::<Vec<Option<String>>>());

        diesel::update(tenant_settings::table.filter(tenant_settings::tenant_id.eq(tenant_id)))
            .set((
                allow_signups.map(|v| tenant_settings::allow_signups.eq(v)),
                require_email_verification
                    .map(|v| tenant_settings::require_email_verification.eq(v)),
                enable_2fa.map(|v| tenant_settings::enable_2fa.eq(v)),
                session_duration_minutes.map(|v| tenant_settings::session_duration_minutes.eq(v)),
                providers_opt.map(|v| tenant_settings::allowed_oauth_providers.eq(v)),
                webhook_url.map(|v| tenant_settings::webhook_url.eq(v)),
                tenant_settings::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn update_email_templates(&self, tenant_id: Uuid, templates: JsonValue) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenant_settings::table.filter(tenant_settings::tenant_id.eq(tenant_id)))
            .set((
                tenant_settings::custom_email_templates.eq(templates),
                tenant_settings::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}
