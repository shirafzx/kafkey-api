use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::api_keys;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = api_keys)]
pub struct ApiKeyEntity {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub environment: Option<String>,
    pub is_active: Option<bool>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKeyEntity {
    pub tenant_id: Uuid,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: String,
    pub environment: Option<String>,
}
