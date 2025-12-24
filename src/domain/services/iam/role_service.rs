use crate::domain::entities::iam::role::{NewRole, Role, RoleInfo, UpdateRole};
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn create_role(&self, new_role: NewRole) -> Result<Role, Box<dyn Error>>;
    async fn get_role_by_id(&self, role_id: i32) -> Result<Option<Role>, Box<dyn Error>>;
    async fn get_role_by_name(&self, name: &str) -> Result<Option<Role>, Box<dyn Error>>;
    async fn get_role_info(&self, role_id: i32) -> Result<Option<RoleInfo>, Box<dyn Error>>;
    async fn update_role(&self, role_id: i32, update_role: UpdateRole) -> Result<Role, Box<dyn Error>>;
    async fn delete_role(&self, role_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn list_roles(&self, limit: i64, offset: i64) -> Result<Vec<RoleInfo>, Box<dyn Error>>;
    async fn assign_permission(&self, role_id: i32, permission_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn revoke_permission(&self, role_id: i32, permission_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn get_role_permissions(&self, role_id: i32) -> Result<Vec<String>, Box<dyn Error>>;
    async fn assign_permission_to_user(&self, user_id: i32, resource: &str, action: &str) -> Result<bool, Box<dyn Error>>;
}
