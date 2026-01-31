use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_image_url: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_image_url: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub roles: Vec<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
    pub avatar_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdminUpdateUserRequest {
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
    pub avatar_image_url: Option<String>,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignRoleRequest {
    pub role_id: uuid::Uuid,
}
