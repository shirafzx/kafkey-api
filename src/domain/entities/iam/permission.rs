use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Option<i32>,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPermission {
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermission {
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionInfo {
    pub id: i32,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAction {
    pub resource: String,
    pub action: String,
}
