use serde::{Deserialize, Serialize};
use validator::Validate;

// ===== Tenant Admin DTOs =====

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTenantAdminRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginTenantAdminRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantAdminResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub company_name: Option<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginTenantAdminResponse {
    pub access_token: String,
    pub admin: TenantAdminResponse,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTenantAdminRequest {
    pub name: Option<String>,
    pub company_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

// ===== Tenant DTOs =====

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub slug: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantResponse {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub slug: String,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
    pub is_active: bool,
    pub plan_tier: String,
    pub max_users: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlanTierRequest {
    #[validate(length(min = 1))]
    pub plan_tier: String,
    #[validate(range(min = 1))]
    pub max_users: i32,
}

// ===== API Key DTOs =====

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1))]
    pub environment: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
    pub id: String,
    pub tenant_id: String,
    pub key_prefix: String,
    pub name: String,
    pub environment: String,
    pub is_active: bool,
    pub last_used_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub id: String,
    pub key: String, // Plain key - only shown once
    pub key_prefix: String,
    pub name: String,
    pub environment: String,
}
