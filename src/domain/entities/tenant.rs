use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::tenants;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = tenants)]
pub struct TenantEntity {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
    pub is_active: Option<bool>,
    pub plan_tier: Option<String>,
    pub max_users: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = tenants)]
pub struct NewTenantEntity {
    pub owner_id: Uuid,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
}
