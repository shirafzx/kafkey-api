use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::role_permissions;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = role_permissions)]
#[diesel(primary_key(role_id, permission_id))]
pub struct RolePermissionEntity {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = role_permissions)]
pub struct NewRolePermissionEntity {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
