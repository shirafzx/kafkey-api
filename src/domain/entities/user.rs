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
    pub failed_login_attempts: i32,
    pub locked_at: Option<DateTime<Utc>>,
    pub verification_token: Option<String>,
    pub verification_token_expires_at: Option<DateTime<Utc>>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    pub two_factor_secret: Option<String>,
    pub two_factor_enabled: bool,
    pub two_factor_backup_codes: Option<Vec<Option<String>>>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserEntity {
    pub username: String,
    pub display_name: String,
    pub avatar_image_url: Option<String>,
    pub password_hash: String,
    pub email: String,
    pub verification_token: Option<String>,
    pub verification_token_expires_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Default)]
pub struct AdminUpdateUserParams {
    pub display_name: Option<String>,
    pub avatar_image_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<Option<String>>,
    pub verification_token_expires_at: Option<Option<DateTime<Utc>>>,
    pub password_reset_token: Option<Option<String>>,
    pub password_reset_expires_at: Option<Option<DateTime<Utc>>>,
    pub two_factor_secret: Option<Option<String>>,
    pub two_factor_enabled: Option<bool>,
    pub two_factor_backup_codes: Option<Option<Vec<Option<String>>>>,
}
