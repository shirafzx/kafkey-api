use crate::domain::entities::iam::permission::{
    NewPermission, Permission, PermissionInfo, UpdatePermission,
};
use crate::domain::services::iam::PermissionService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionResponse {
    pub permission: Permission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
    pub id: i32,
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionResponse {
    pub permission: Permission,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionResponse {
    pub permission_info: PermissionInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<PermissionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionRequest {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllResourcesResponse {
    pub resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActionsForResourceRequest {
    pub resource: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetActionsForResourceResponse {
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionRequest {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionResponse {
    pub has_permission: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPermissionsResponse {
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRolePermissionsResponse {
    pub permissions: Vec<String>,
}

#[async_trait]
pub trait PermissionUseCases: Send + Sync {
    async fn create_permission(
        &self,
        request: CreatePermissionRequest,
    ) -> Result<CreatePermissionResponse, Box<dyn Error>>;
    async fn get_permission(
        &self,
        request: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, Box<dyn Error>>;
    async fn update_permission(
        &self,
        request: UpdatePermissionRequest,
    ) -> Result<UpdatePermissionResponse, Box<dyn Error>>;
    async fn delete_permission(
        &self,
        request: DeletePermissionRequest,
    ) -> Result<DeletePermissionResponse, Box<dyn Error>>;
    async fn list_permissions(
        &self,
        request: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, Box<dyn Error>>;
    async fn get_all_resources(&self) -> Result<GetAllResourcesResponse, Box<dyn Error>>;
    async fn get_actions_for_resource(
        &self,
        request: GetActionsForResourceRequest,
    ) -> Result<GetActionsForResourceResponse, Box<dyn Error>>;
    async fn check_permission(
        &self,
        request: CheckPermissionRequest,
    ) -> Result<CheckPermissionResponse, Box<dyn Error>>;
    async fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Result<GetUserPermissionsResponse, Box<dyn Error>>;
    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<GetRolePermissionsResponse, Box<dyn Error>>;
}

pub struct PermissionUseCasesImpl<T: PermissionService> {
    permission_service: T,
}

impl<T: PermissionService> PermissionUseCasesImpl<T> {
    pub fn new(permission_service: T) -> Self {
        Self { permission_service }
    }
}

#[async_trait]
impl<T: PermissionService> PermissionUseCases for PermissionUseCasesImpl<T> {
    async fn create_permission(
        &self,
        request: CreatePermissionRequest,
    ) -> Result<CreatePermissionResponse, Box<dyn Error>> {
        let new_permission = NewPermission {
            name: request.name,
            resource: request.resource,
            action: request.action,
            description: request.description,
        };

        let permission = self
            .permission_service
            .create_permission(new_permission)
            .await?;

        Ok(CreatePermissionResponse { permission })
    }

    async fn get_permission(
        &self,
        request: GetPermissionRequest,
    ) -> Result<GetPermissionResponse, Box<dyn Error>> {
        let permission_info = match (request.id, request.name, (request.resource, request.action)) {
            (Some(id), _, _) => self
                .permission_service
                .get_permission_info(id)
                .await?
                .ok_or("Permission not found")?,
            (_, Some(name), _) => {
                let permission = self
                    .permission_service
                    .get_permission_by_name(&name)
                    .await?
                    .ok_or("Permission not found")?;
                self.permission_service
                    .get_permission_info(permission.id.ok_or("Invalid permission")?)
                    .await?
                    .ok_or("Permission info not found")?
            }
            (_, _, (Some(resource), Some(action))) => {
                let permission = self
                    .permission_service
                    .get_permission_by_resource_action(&resource, &action)
                    .await?
                    .ok_or("Permission not found")?;
                self.permission_service
                    .get_permission_info(permission.id.ok_or("Invalid permission")?)
                    .await?
                    .ok_or("Permission info not found")?
            }
            _ => {
                return Err(
                    "At least one identifier (id, name, or resource+action) must be provided"
                        .into(),
                );
            }
        };

        Ok(GetPermissionResponse { permission_info })
    }

    async fn update_permission(
        &self,
        request: UpdatePermissionRequest,
    ) -> Result<UpdatePermissionResponse, Box<dyn Error>> {
        let update_permission = UpdatePermission {
            name: request.name,
            resource: request.resource,
            action: request.action,
            description: request.description,
        };

        let permission = self
            .permission_service
            .update_permission(request.id, update_permission)
            .await?;

        Ok(UpdatePermissionResponse { permission })
    }

    async fn delete_permission(
        &self,
        request: DeletePermissionRequest,
    ) -> Result<DeletePermissionResponse, Box<dyn Error>> {
        let success = self
            .permission_service
            .delete_permission(request.id)
            .await?;

        Ok(DeletePermissionResponse { success })
    }

    async fn list_permissions(
        &self,
        request: ListPermissionsRequest,
    ) -> Result<ListPermissionsResponse, Box<dyn Error>> {
        let limit = request.limit.unwrap_or(20);
        let offset = request.offset.unwrap_or(0);

        let permissions = self
            .permission_service
            .list_permissions(limit, offset)
            .await?;

        Ok(ListPermissionsResponse { permissions })
    }

    async fn get_all_resources(&self) -> Result<GetAllResourcesResponse, Box<dyn Error>> {
        let resources = self.permission_service.get_all_resources().await?;

        Ok(GetAllResourcesResponse { resources })
    }

    async fn get_actions_for_resource(
        &self,
        request: GetActionsForResourceRequest,
    ) -> Result<GetActionsForResourceResponse, Box<dyn Error>> {
        let actions = self
            .permission_service
            .get_actions_for_resource(&request.resource)
            .await?;

        Ok(GetActionsForResourceResponse { actions })
    }

    async fn check_permission(
        &self,
        request: CheckPermissionRequest,
    ) -> Result<CheckPermissionResponse, Box<dyn Error>> {
        let has_permission = self
            .permission_service
            .check_permission(request.user_id, &request.resource, &request.action)
            .await?;

        Ok(CheckPermissionResponse { has_permission })
    }

    async fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Result<GetUserPermissionsResponse, Box<dyn Error>> {
        let permissions = self
            .permission_service
            .get_user_permissions(user_id)
            .await?;

        Ok(GetUserPermissionsResponse { permissions })
    }

    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<GetRolePermissionsResponse, Box<dyn Error>> {
        let permissions = self
            .permission_service
            .get_role_permissions(role_id)
            .await?;

        Ok(GetRolePermissionsResponse { permissions })
    }
}
