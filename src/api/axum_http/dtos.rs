use serde::{Deserialize, Serialize};

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub avatar_image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub avatar_image_url: Option<String>,
}
