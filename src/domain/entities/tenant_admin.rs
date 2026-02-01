use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::tenant_admins;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = tenant_admins)]
pub struct TenantAdminEntity {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = tenant_admins)]
pub struct NewTenantAdminEntity {
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
}
