use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::user_roles;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = user_roles)]
#[diesel(primary_key(user_id, role_id))]
pub struct UserRoleEntity {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_roles)]
pub struct NewUserRoleEntity {
    pub user_id: Uuid,
    pub role_id: Uuid,
}
