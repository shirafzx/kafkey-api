use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePermissionRequest {
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePermissionRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub id: String,
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}
