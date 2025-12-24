use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: bool,
    pub roles: Vec<String>,
}
