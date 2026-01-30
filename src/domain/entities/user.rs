use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::database::postgres::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_image_url: Option<String>,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub email: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub username: String,
    pub display_name: String,
    pub avatar_image_url: Option<String>,
    pub password_hash: String,
    pub email: String,
}
