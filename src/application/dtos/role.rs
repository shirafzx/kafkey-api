use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignPermissionRequest {
    pub permission_id: uuid::Uuid,
}
