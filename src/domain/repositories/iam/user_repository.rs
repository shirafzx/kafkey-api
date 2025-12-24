use crate::domain::entities::iam::user::{NewUser, UpdateUser, User, UserInfo};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, new_user: NewUser) -> Result<User, Box<dyn std::error::Error>>;
    async fn get_user_by_id(
        &self,
        user_id: i32,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn get_user_info(
        &self,
        user_id: i32,
    ) -> Result<Option<UserInfo>, Box<dyn std::error::Error>>;
    async fn update_user(
        &self,
        user_id: i32,
        update_user: UpdateUser,
    ) -> Result<User, Box<dyn std::error::Error>>;
    async fn delete_user(&self, user_id: i32) -> Result<bool, Box<dyn std::error::Error>>;
    async fn list_users(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserInfo>, Box<dyn std::error::Error>>;
    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>>;
}
