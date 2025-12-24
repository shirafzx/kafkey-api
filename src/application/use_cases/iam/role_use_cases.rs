use crate::domain::entities::iam::role::{NewRole, Role, RoleInfo, UpdateRole};
use crate::domain::services::iam::RoleService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleResponse {
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleResponse {
    pub role_info: RoleInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesResponse {
    pub roles: Vec<RoleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleRequest {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoleResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignPermissionResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionRequest {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokePermissionResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolePermissionsResponse {
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignPermissionToUserRequest {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignPermissionToUserResponse {
    pub success: bool,
}

#[async_trait]
pub trait RoleUseCases: Send + Sync {
    async fn create_role(
        &self,
        request: CreateRoleRequest,
    ) -> Result<CreateRoleResponse, Box<dyn Error>>;
    async fn get_role(&self, request: GetRoleRequest) -> Result<GetRoleResponse, Box<dyn Error>>;
    async fn update_role(
        &self,
        request: UpdateRoleRequest,
    ) -> Result<UpdateRoleResponse, Box<dyn Error>>;
    async fn delete_role(
        &self,
        request: DeleteRoleRequest,
    ) -> Result<DeleteRoleResponse, Box<dyn Error>>;
    async fn list_roles(
        &self,
        request: ListRolesRequest,
    ) -> Result<ListRolesResponse, Box<dyn Error>>;
    async fn assign_permission(
        &self,
        request: AssignPermissionRequest,
    ) -> Result<AssignPermissionResponse, Box<dyn Error>>;
    async fn revoke_permission(
        &self,
        request: RevokePermissionRequest,
    ) -> Result<RevokePermissionResponse, Box<dyn Error>>;
    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<GetRolePermissionsResponse, Box<dyn Error>>;
    async fn assign_permission_to_user(
        &self,
        request: AssignPermissionToUserRequest,
    ) -> Result<AssignPermissionToUserResponse, Box<dyn Error>>;
}

pub struct RoleUseCasesImpl<T: RoleService> {
    role_service: T,
}

impl<T: RoleService> RoleUseCasesImpl<T> {
    pub fn new(role_service: T) -> Self {
        Self { role_service }
    }
}

#[async_trait]
impl<T: RoleService> RoleUseCases for RoleUseCasesImpl<T> {
    async fn create_role(
        &self,
        request: CreateRoleRequest,
    ) -> Result<CreateRoleResponse, Box<dyn Error>> {
        let new_role = NewRole {
            name: request.name,
            description: request.description,
        };

        let role = self.role_service.create_role(new_role).await?;

        Ok(CreateRoleResponse { role })
    }

    async fn get_role(&self, request: GetRoleRequest) -> Result<GetRoleResponse, Box<dyn Error>> {
        let role_info = match (request.id, request.name) {
            (Some(id), _) => self
                .role_service
                .get_role_info(id)
                .await?
                .ok_or("Role not found")?,
            (_, Some(name)) => {
                let role = self
                    .role_service
                    .get_role_by_name(&name)
                    .await?
                    .ok_or("Role not found")?;
                self.role_service
                    .get_role_info(role.id.ok_or("Invalid role")?)
                    .await?
                    .ok_or("Role info not found")?
            }
            _ => {
                return Err("At least one identifier (id or name) must be provided".into());
            }
        };

        Ok(GetRoleResponse { role_info })
    }

    async fn update_role(
        &self,
        request: UpdateRoleRequest,
    ) -> Result<UpdateRoleResponse, Box<dyn Error>> {
        let update_role = UpdateRole {
            name: request.name,
            description: request.description,
        };

        let role = self
            .role_service
            .update_role(request.id, update_role)
            .await?;

        Ok(UpdateRoleResponse { role })
    }

    async fn delete_role(
        &self,
        request: DeleteRoleRequest,
    ) -> Result<DeleteRoleResponse, Box<dyn Error>> {
        let success = self.role_service.delete_role(request.id).await?;

        Ok(DeleteRoleResponse { success })
    }

    async fn list_roles(
        &self,
        request: ListRolesRequest,
    ) -> Result<ListRolesResponse, Box<dyn Error>> {
        let limit = request.limit.unwrap_or(20);
        let offset = request.offset.unwrap_or(0);

        let roles = self.role_service.list_roles(limit, offset).await?;

        Ok(ListRolesResponse { roles })
    }

    async fn assign_permission(
        &self,
        request: AssignPermissionRequest,
    ) -> Result<AssignPermissionResponse, Box<dyn Error>> {
        let success = self
            .role_service
            .assign_permission(request.role_id, request.permission_id)
            .await?;

        Ok(AssignPermissionResponse { success })
    }

    async fn revoke_permission(
        &self,
        request: RevokePermissionRequest,
    ) -> Result<RevokePermissionResponse, Box<dyn Error>> {
        let success = self
            .role_service
            .revoke_permission(request.role_id, request.permission_id)
            .await?;

        Ok(RevokePermissionResponse { success })
    }

    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<GetRolePermissionsResponse, Box<dyn Error>> {
        let permissions = self.role_service.get_role_permissions(role_id).await?;

        Ok(GetRolePermissionsResponse { permissions })
    }

    async fn assign_permission_to_user(
        &self,
        request: AssignPermissionToUserRequest,
    ) -> Result<AssignPermissionToUserResponse, Box<dyn Error>> {
        let success = self
            .role_service
            .assign_permission_to_user(request.user_id, &request.resource, &request.action)
            .await?;

        Ok(AssignPermissionToUserResponse { success })
    }
}
