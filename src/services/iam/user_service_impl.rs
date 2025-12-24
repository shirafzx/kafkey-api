use crate::domain::entities::iam::user::{NewUser, UpdateUser, User, UserInfo};
use crate::domain::repositories::iam::{PermissionRepository, UserRepository, UserRoleRepository};
use crate::domain::services::iam::UserService;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct UserServiceImpl<T: UserRepository, U: UserRoleRepository, V: PermissionRepository> {
    user_repository: Arc<T>,
    user_role_repository: Arc<U>,
    permission_repository: Arc<V>,
}

impl<T: UserRepository, U: UserRoleRepository, V: PermissionRepository> UserServiceImpl<T, U, V> {
    pub fn new(
        user_repository: Arc<T>,
        user_role_repository: Arc<U>,
        permission_repository: Arc<V>,
    ) -> Self {
        Self {
            user_repository,
            user_role_repository,
            permission_repository,
        }
    }
}

#[async_trait]
impl<T: UserRepository, U: UserRoleRepository, V: PermissionRepository> UserService
    for UserServiceImpl<T, U, V>
{
    async fn create_user(&self, new_user: NewUser) -> Result<User, Box<dyn Error>> {
        self.user_repository.create_user(new_user).await
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.get_user_by_id(user_id).await
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.get_user_by_username(username).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository.get_user_by_email(email).await
    }

    async fn get_user_info(&self, user_id: i32) -> Result<Option<UserInfo>, Box<dyn Error>> {
        self.user_repository.get_user_info(user_id).await
    }

    async fn update_user(
        &self,
        user_id: i32,
        update_user: UpdateUser,
    ) -> Result<User, Box<dyn Error>> {
        self.user_repository.update_user(user_id, update_user).await
    }

    async fn delete_user(&self, user_id: i32) -> Result<bool, Box<dyn Error>> {
        self.user_repository.delete_user(user_id).await
    }

    async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<UserInfo>, Box<dyn Error>> {
        self.user_repository.list_users(limit, offset).await
    }

    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        self.user_repository
            .authenticate_user(username, password)
            .await
    }

    async fn assign_role(&self, user_id: i32, role_id: i32) -> Result<bool, Box<dyn Error>> {
        // Check if user exists
        let user = self.user_repository.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Err("User not found".into());
        }

        // Create the user-role association
        let new_user_role =
            crate::domain::entities::iam::user_role::NewUserRole { user_id, role_id };
        self.user_role_repository
            .assign_role_to_user(new_user_role)
            .await?;
        Ok(true)
    }

    async fn revoke_role(&self, user_id: i32, role_id: i32) -> Result<bool, Box<dyn Error>> {
        // Check if user exists
        let user = self.user_repository.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Err("User not found".into());
        }

        // Remove the user-role association
        self.user_role_repository
            .remove_role_from_user(user_id, role_id)
            .await
    }

    async fn has_permission(
        &self,
        user_id: i32,
        resource: &str,
        action: &str,
    ) -> Result<bool, Box<dyn Error>> {
        // Check if user exists
        let user = self.user_repository.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Err("User not found".into());
        }

        // Get user's permissions
        let user_permissions = self.get_user_permissions(user_id).await?;

        // Check if the requested permission exists in the user's permissions
        let requested_permission = format!("{}:{}", resource, action);
        Ok(user_permissions.contains(&requested_permission))
    }

    async fn has_role(&self, user_id: i32, role_name: &str) -> Result<bool, Box<dyn Error>> {
        // Check if user exists
        let user = self.user_repository.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Err("User not found".into());
        }

        // Get user's roles
        let user_roles = self.get_user_roles(user_id).await?;

        // Check if the requested role exists in the user's roles
        Ok(user_roles.contains(&role_name.to_string()))
    }

    async fn get_user_permissions(&self, user_id: i32) -> Result<Vec<String>, Box<dyn Error>> {
        // Get user's roles
        let user_roles_info = self.user_role_repository.get_user_roles(user_id).await?;

        // Collect all permissions for all user's roles
        let mut all_permissions = Vec::new();
        for role_info in user_roles_info {
            let role_permissions = self
                .permission_repository
                .get_role_permissions(role_info.role_id)
                .await?;

            // Format permissions as "resource:action"
            let formatted_permissions: Vec<String> = role_permissions
                .into_iter()
                .map(|p| format!("{}:{}", p.resource, p.action))
                .collect();

            all_permissions.extend(formatted_permissions);
        }

        // Remove duplicates and return
        all_permissions.sort();
        all_permissions.dedup();
        Ok(all_permissions)
    }

    async fn get_user_roles(&self, user_id: i32) -> Result<Vec<String>, Box<dyn Error>> {
        // Get user's roles
        let user_roles_info = self.user_role_repository.get_user_roles(user_id).await?;

        // Extract role names
        let role_names: Vec<String> = user_roles_info.into_iter().map(|r| r.role_name).collect();

        Ok(role_names)
    }
}
