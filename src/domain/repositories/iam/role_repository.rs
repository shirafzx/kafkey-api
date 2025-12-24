use crate::domain::entities::iam::role::{NewRole, Role, RoleInfo, UpdateRole};
use async_trait::async_trait;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create_role(&self, new_role: NewRole) -> Result<Role, Box<dyn std::error::Error>>;
    async fn get_role_by_id(
        &self,
        role_id: i32,
    ) -> Result<Option<Role>, Box<dyn std::error::Error>>;
    async fn get_role_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Role>, Box<dyn std::error::Error>>;
    async fn get_role_info(
        &self,
        role_id: i32,
    ) -> Result<Option<RoleInfo>, Box<dyn std::error::Error>>;
    async fn update_role(
        &self,
        role_id: i32,
        update_role: UpdateRole,
    ) -> Result<Role, Box<dyn std::error::Error>>;
    async fn delete_role(&self, role_id: i32) -> Result<bool, Box<dyn std::error::Error>>;
    async fn list_roles(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RoleInfo>, Box<dyn std::error::Error>>;
    async fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Result<Vec<RoleInfo>, Box<dyn std::error::Error>>;
}
