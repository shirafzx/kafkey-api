use crate::domain::entities::iam::role_permission::{
    NewRolePermission, RolePermission, RolePermissionInfo,
};
use async_trait::async_trait;

#[async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn assign_permission_to_role(
        &self,
        new_role_permission: NewRolePermission,
    ) -> Result<RolePermission, Box<dyn std::error::Error>>;
    async fn remove_permission_from_role(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_role_permission(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<Option<RolePermission>, Box<dyn std::error::Error>>;
    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<Vec<RolePermissionInfo>, Box<dyn std::error::Error>>;
    async fn get_permission_roles(
        &self,
        permission_id: i32,
    ) -> Result<Vec<RolePermissionInfo>, Box<dyn std::error::Error>>;
    async fn remove_all_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
