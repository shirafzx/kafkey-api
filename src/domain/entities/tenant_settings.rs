use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::tenant_settings;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = tenant_settings)]
pub struct TenantSettingsEntity {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub allow_signups: Option<bool>,
    pub require_email_verification: Option<bool>,
    pub enable_2fa: Option<bool>,
    pub session_duration_minutes: Option<i32>,
    pub allowed_oauth_providers: Option<Vec<Option<String>>>,
    pub custom_email_templates: Option<JsonValue>,
    pub webhook_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = tenant_settings)]
pub struct NewTenantSettingsEntity {
    pub tenant_id: Uuid,
}
