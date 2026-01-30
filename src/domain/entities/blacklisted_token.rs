use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::blacklisted_tokens;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = blacklisted_tokens)]
#[diesel(primary_key(jti))]
pub struct BlacklistedTokenEntity {
    pub jti: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = blacklisted_tokens)]
pub struct NewBlacklistedTokenEntity {
    pub jti: Uuid,
    pub expires_at: DateTime<Utc>,
}
