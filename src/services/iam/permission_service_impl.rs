use crate::domain::entities::iam::permission::{
    NewPermission, Permission, PermissionInfo, ResourceAction, UpdatePermission,
};
use crate::domain::entities::iam::user_role::NewUserRole;
use crate::domain::repositories::iam::{
    PermissionRepository, RolePermissionRepository, RoleRepository, UserRepository,
    UserRoleRepository,
};
use crate::domain::services::iam::PermissionService;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct PermissionServiceImpl<
    T: PermissionRepository,
    U: RolePermissionRepository,
    V: RoleRepository,
    W: UserRoleRepository,
    X: UserRepository,
> {
    permission_repository: Arc<T>,
    role_permission_repository: Arc<U>,
    role_repository: Arc<V>,
    user_role_repository: Arc<W>,
    user_repository: Arc<X>,
}

impl<
    T: PermissionRepository,
    U: RolePermissionRepository,
    V: RoleRepository,
    W: UserRoleRepository,
    X: UserRepository,
> PermissionServiceImpl<T, U, V, W, X>
{
    pub fn new(
        permission_repository: Arc<T>,
        role_permission_repository: Arc<U>,
        role_repository: Arc<V>,
        user_role_repository: Arc<W>,
        user_repository: Arc<X>,
    ) -> Self {
        Self {
            permission_repository,
            role_permission_repository,
            role_repository,
            user_role_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl<
    T: PermissionRepository,
    U: RolePermissionRepository,
    V: RoleRepository,
    W: UserRoleRepository,
    X: UserRepository,
> PermissionService for PermissionServiceImpl<T, U, V, W, X>
{
    async fn create_permission(
        &self,
        new_permission: NewPermission,
    ) -> Result<Permission, Box<dyn Error>> {
        self.permission_repository
            .create_permission(new_permission)
            .await
    }

    async fn get_permission_by_id(
        &self,
        permission_id: i32,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        self.permission_repository
            .get_permission_by_id(permission_id)
            .await
    }

    async fn get_permission_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        self.permission_repository
            .get_permission_by_name(name)
            .await
    }

    async fn get_permission_by_resource_action(
        &self,
        resource: &str,
        action: &str,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        self.permission_repository
            .get_permission_by_resource_action(resource, action)
            .await
    }

    async fn get_permission_info(
        &self,
        permission_id: i32,
    ) -> Result<Option<PermissionInfo>, Box<dyn Error>> {
        self.permission_repository
            .get_permission_info(permission_id)
            .await
    }

    async fn update_permission(
        &self,
        permission_id: i32,
        update_permission: UpdatePermission,
    ) -> Result<Permission, Box<dyn Error>> {
        self.permission_repository
            .update_permission(permission_id, update_permission)
            .await
    }

    async fn delete_permission(&self, permission_id: i32) -> Result<bool, Box<dyn Error>> {
        self.permission_repository
            .delete_permission(permission_id)
            .await
    }

    async fn list_permissions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PermissionInfo>, Box<dyn Error>> {
        self.permission_repository
            .list_permissions(limit, offset)
            .await
    }

    async fn get_all_resources(&self) -> Result<Vec<String>, Box<dyn Error>> {
        // Get all permissions
        let permissions = self.permission_repository.list_permissions(1000, 0).await?;

        // Extract unique resources
        let mut resources = std::collections::HashSet::new();
        for permission in permissions {
            resources.insert(permission.resource);
        }

        // Convert to sorted vector
        let mut result: Vec<String> = resources.into_iter().collect();
        result.sort();

        Ok(result)
    }

    async fn get_actions_for_resource(
        &self,
        resource: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        // Get all permissions for the resource
        let all_permissions = self.permission_repository.list_permissions(1000, 0).await?;

        // Filter by resource and extract unique actions
        let mut actions = std::collections::HashSet::new();
        for permission in all_permissions {
            if permission.resource == resource {
                actions.insert(permission.action);
            }
        }

        // Convert to sorted vector
        let mut result: Vec<String> = actions.into_iter().collect();
        result.sort();

        Ok(result)
    }

    async fn check_permission(
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

        // Get user's roles
        let user_roles = self.user_role_repository.get_user_roles(user_id).await?;

        // For each role, check if it has the requested permission
        for user_role in user_roles {
            let role_permissions = self
                .role_permission_repository
                .get_role_permissions(user_role.role_id)
                .await?;

            for role_permission in role_permissions {
                if role_permission.resource == resource && role_permission.action == action {
                    return Ok(true);
                }
            }
        }

        // No matching permission found
        Ok(false)
    }

    async fn get_user_permissions(&self, user_id: i32) -> Result<Vec<String>, Box<dyn Error>> {
        // Check if user exists
        let user = self.user_repository.get_user_by_id(user_id).await?;
        if user.is_none() {
            return Err("User not found".into());
        }

        // Get user's roles
        let user_roles = self.user_role_repository.get_user_roles(user_id).await?;

        // Collect all permissions from all user's roles
        let mut all_permissions = std::collections::HashSet::new();
        for user_role in user_roles {
            let role_permissions = self
                .role_permission_repository
                .get_role_permissions(user_role.role_id)
                .await?;

            for role_permission in role_permissions {
                let permission_str =
                    format!("{}:{}", role_permission.resource, role_permission.action);
                all_permissions.insert(permission_str);
            }
        }

        // Convert to sorted vector
        let mut result: Vec<String> = all_permissions.into_iter().collect();
        result.sort();

        Ok(result)
    }

    async fn get_role_permissions(&self, role_id: i32) -> Result<Vec<String>, Box<dyn Error>> {
        // Check if role exists
        let role = self.role_repository.get_role_by_id(role_id).await?;
        if role.is_none() {
            return Err("Role not found".into());
        }

        // Get role's permissions
        let role_permissions = self
            .role_permission_repository
            .get_role_permissions(role_id)
            .await?;

        // Format permissions as "resource:action"
        let permissions: Vec<String> = role_permissions
            .into_iter()
            .map(|p| format!("{}:{}", p.resource, p.action))
            .collect();

        Ok(permissions)
    }
}
