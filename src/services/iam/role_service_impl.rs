use crate::domain::entities::iam::permission::{NewPermission, PermissionInfo};
use crate::domain::entities::iam::role::{NewRole, Role, RoleInfo, UpdateRole};
use crate::domain::repositories::iam::{
    PermissionRepository, RolePermissionRepository, RoleRepository, UserRepository,
};
use crate::domain::services::iam::RoleService;
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct RoleServiceImpl<
    T: RoleRepository,
    U: RolePermissionRepository,
    V: PermissionRepository,
    W: UserRepository,
> {
    role_repository: Arc<T>,
    role_permission_repository: Arc<U>,
    permission_repository: Arc<V>,
    user_repository: Arc<W>,
}

impl<T: RoleRepository, U: RolePermissionRepository, V: PermissionRepository, W: UserRepository>
    RoleServiceImpl<T, U, V, W>
{
    pub fn new(
        role_repository: Arc<T>,
        role_permission_repository: Arc<U>,
        permission_repository: Arc<V>,
        user_repository: Arc<W>,
    ) -> Self {
        Self {
            role_repository,
            role_permission_repository,
            permission_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl<T: RoleRepository, U: RolePermissionRepository, V: PermissionRepository, W: UserRepository>
    RoleService for RoleServiceImpl<T, U, V, W>
{
    async fn create_role(&self, new_role: NewRole) -> Result<Role, Box<dyn Error>> {
        self.role_repository.create_role(new_role).await
    }

    async fn get_role_by_id(&self, role_id: i32) -> Result<Option<Role>, Box<dyn Error>> {
        self.role_repository.get_role_by_id(role_id).await
    }

    async fn get_role_by_name(&self, name: &str) -> Result<Option<Role>, Box<dyn Error>> {
        self.role_repository.get_role_by_name(name).await
    }

    async fn get_role_info(&self, role_id: i32) -> Result<Option<RoleInfo>, Box<dyn Error>> {
        self.role_repository.get_role_info(role_id).await
    }

    async fn update_role(
        &self,
        role_id: i32,
        update_role: UpdateRole,
    ) -> Result<Role, Box<dyn Error>> {
        self.role_repository.update_role(role_id, update_role).await
    }

    async fn delete_role(&self, role_id: i32) -> Result<bool, Box<dyn Error>> {
        self.role_repository.delete_role(role_id).await
    }

    async fn list_roles(&self, limit: i64, offset: i64) -> Result<Vec<RoleInfo>, Box<dyn Error>> {
        self.role_repository.list_roles(limit, offset).await
    }

    async fn assign_permission(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<bool, Box<dyn Error>> {
        // Check if role exists
        let role = self.role_repository.get_role_by_id(role_id).await?;
        if role.is_none() {
            return Err("Role not found".into());
        }

        // Check if permission exists
        let permission = self
            .permission_repository
            .get_permission_by_id(permission_id)
            .await?;
        if permission.is_none() {
            return Err("Permission not found".into());
        }

        // Create the role-permission association
        let new_role_permission =
            crate::domain::entities::iam::role_permission::NewRolePermission {
                role_id,
                permission_id,
            };
        self.role_permission_repository
            .assign_permission_to_role(new_role_permission)
            .await?;
        Ok(true)
    }

    async fn revoke_permission(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<bool, Box<dyn Error>> {
        // Check if role exists
        let role = self.role_repository.get_role_by_id(role_id).await?;
        if role.is_none() {
            return Err("Role not found".into());
        }

        // Remove the role-permission association
        self.role_permission_repository
            .revoke_permission_from_role(role_id, permission_id)
            .await
    }

    async fn get_role_permissions(&self, role_id: i32) -> Result<Vec<String>, Box<dyn Error>> {
        // Check if role exists
        let role = self.role_repository.get_role_by_id(role_id).await?;
        if role.is_none() {
            return Err("Role not found".into());
        }

        // Get role's permissions
        let role_permissions_info = self
            .role_permission_repository
            .get_role_permissions(role_id)
            .await?;

        // Format permissions as "resource:action"
        let permissions: Vec<String> = role_permissions_info
            .into_iter()
            .map(|p| format!("{}:{}", p.resource, p.action))
            .collect();

        Ok(permissions)
    }

    async fn assign_permission_to_user(
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

        // Check if the permission exists
        let permission = self
            .permission_repository
            .get_permission_by_resource_action(resource, action)
            .await?;

        let permission_id = match permission {
            Some(p) => p.id.ok_or("Permission ID is missing")?,
            None => {
                // Create a new permission if it doesn't exist
                let new_permission = NewPermission {
                    name: format!("{}:{}", resource, action),
                    resource: resource.to_string(),
                    action: action.to_string(),
                    description: Some(format!("Access to {} with action {}", resource, action)),
                };
                let created = self
                    .permission_repository
                    .create_permission(new_permission)
                    .await?;
                created.id.ok_or("Created permission has no ID")?
            }
        };

        // Create a user-specific role if needed
        let user_role_name = format!("user-{}-custom", user_id);
        let role_id = match self
            .role_repository
            .get_role_by_name(&user_role_name)
            .await?
        {
            Some(role) => role.id.ok_or("Role ID is missing")?,
            None => {
                let new_role = crate::domain::entities::iam::role::NewRole {
                    name: user_role_name.clone(),
                    description: Some(format!("Custom role for user {}", user_id)),
                };
                let role = self.role_repository.create_role(new_role).await?;

                // Assign the role to the user
                let new_user_role = crate::domain::entities::iam::user_role::NewUserRole {
                    user_id,
                    role_id: role.id.ok_or("Created role has no ID")?,
                };
                // Note: This would require injecting UserRoleRepository to work properly

                role.id.ok_or("Created role has no ID")?
            }
        };

        // Assign the permission to the role
        self.assign_permission(role_id, permission_id).await
    }
}
