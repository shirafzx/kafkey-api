use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<i32>,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleInfo {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub username: String,
    pub role_name: String,
}
