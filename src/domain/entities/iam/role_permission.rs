use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub id: Option<i32>,
    pub role_id: i32,
    pub permission_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermissionInfo {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
    pub role_name: String,
    pub permission_name: String,
    pub resource: String,
    pub action: String,
}
