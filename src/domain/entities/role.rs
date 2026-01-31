use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::roles;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = roles)]
pub struct RoleEntity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = roles)]
pub struct NewRoleEntity {
    pub name: String,
    pub description: Option<String>,
}
