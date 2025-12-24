use crate::domain::entities::iam::user_role::{NewUserRole, UserRole, UserRoleInfo};
use async_trait::async_trait;

#[async_trait]
pub trait UserRoleRepository: Send + Sync {
    async fn assign_role_to_user(
        &self,
        new_user_role: NewUserRole,
    ) -> Result<UserRole, Box<dyn std::error::Error>>;
    async fn remove_role_from_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_user_role(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<Option<UserRole>, Box<dyn std::error::Error>>;
    async fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Result<Vec<UserRoleInfo>, Box<dyn std::error::Error>>;
    async fn get_role_users(
        &self,
        role_id: i32,
    ) -> Result<Vec<UserRoleInfo>, Box<dyn std::error::Error>>;
    async fn remove_all_user_roles(&self, user_id: i32)
    -> Result<bool, Box<dyn std::error::Error>>;
}
