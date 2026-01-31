use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub display_name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub avatar_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LoginResponse {
    Success(AuthResponse),
    RequiresMfa { user_id: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyMfaRequest {
    pub user_id: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmMfaRequest {
    pub secret: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disable2faRequest {
    pub code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_register_request_validation() {
        let valid_request = RegisterRequest {
            username: "validuser".to_string(),
            email: "test@example.com".to_string(),
            display_name: "Test User".to_string(),
            password: "password123".to_string(),
            avatar_image_url: None,
        };
        assert!(valid_request.validate().is_ok());

        let invalid_email = RegisterRequest {
            email: "invalid-email".to_string(),
            ..valid_request.clone()
        };
        assert!(invalid_email.validate().is_err());

        let short_password = RegisterRequest {
            password: "short".to_string(),
            ..valid_request.clone()
        };
        assert!(short_password.validate().is_err());

        let short_username = RegisterRequest {
            username: "ab".to_string(),
            ..valid_request.clone()
        };
        assert!(short_username.validate().is_err());
    }
}
