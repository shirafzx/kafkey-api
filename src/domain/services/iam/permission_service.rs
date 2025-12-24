use crate::domain::entities::iam::permission::{NewPermission, Permission, PermissionInfo, UpdatePermission};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait PermissionService: Send + Sync {
    async fn create_permission(&self, new_permission: NewPermission) -> Result<Permission, Box<dyn Error>>;
    async fn get_permission_by_id(&self, permission_id: i32) -> Result<Option<Permission>, Box<dyn Error>>;
    async fn get_permission_by_name(&self, name: &str) -> Result<Option<Permission>, Box<dyn Error>>;
    async fn get_permission_by_resource_action(&self, resource: &str, action: &str) -> Result<Option<Permission>, Box<dyn Error>>;
    async fn get_permission_info(&self, permission_id: i32) -> Result<Option<PermissionInfo>, Box<dyn Error>>;
    async fn update_permission(&self, permission_id: i32, update_permission: UpdatePermission) -> Result<Permission, Box<dyn Error>>;
    async fn delete_permission(&self, permission_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn list_permissions(&self, limit: i64, offset: i64) -> Result<Vec<PermissionInfo>, Box<dyn Error>>;
    async fn get_all_resources(&self) -> Result<Vec<String>, Box<dyn Error>>;
    async fn get_actions_for_resource(&self, resource: &str) -> Result<Vec<String>, Box<dyn Error>>;
    async fn check_permission(&self, user_id: i32, resource: &str, action: &str) -> Result<bool, Box<dyn Error>>;
    async fn get_user_permissions(&self, user_id: i32) -> Result<Vec<String>, Box<dyn Error>>;
    async fn get_role_permissions(&self, role_id: i32) -> Result<Vec<String>, Box<dyn Error>>;
}
