use crate::domain::entities::iam::permission::{
    NewPermission, Permission, PermissionInfo, UpdatePermission,
};
use async_trait::async_trait;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create_permission(
        &self,
        new_permission: NewPermission,
    ) -> Result<Permission, Box<dyn std::error::Error>>;
    async fn get_permission_by_id(
        &self,
        permission_id: i32,
    ) -> Result<Option<Permission>, Box<dyn std::error::Error>>;
    async fn get_permission_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Permission>, Box<dyn std::error::Error>>;
    async fn get_permission_by_resource_action(
        &self,
        resource: &str,
        action: &str,
    ) -> Result<Option<Permission>, Box<dyn std::error::Error>>;
    async fn get_permission_info(
        &self,
        permission_id: i32,
    ) -> Result<Option<PermissionInfo>, Box<dyn std::error::Error>>;
    async fn update_permission(
        &self,
        permission_id: i32,
        update_permission: UpdatePermission,
    ) -> Result<Permission, Box<dyn std::error::Error>>;
    async fn delete_permission(
        &self,
        permission_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>>;
    async fn list_permissions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PermissionInfo>, Box<dyn std::error::Error>>;
    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<Vec<PermissionInfo>, Box<dyn std::error::Error>>;
}
