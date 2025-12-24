use crate::domain::entities::iam::user::{NewUser, UpdateUser, User, UserInfo};
use crate::domain::services::iam::UserService;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user_info: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRoleRequest {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRoleResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionRequest {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionResponse {
    pub has_permission: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRolesResponse {
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPermissionsResponse {
    pub permissions: Vec<String>,
}

#[async_trait]
pub trait UserUseCases: Send + Sync {
    async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, Box<dyn Error>>;
    async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, Box<dyn Error>>;
    async fn update_user(
        &self,
        request: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, Box<dyn Error>>;
    async fn delete_user(
        &self,
        request: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, Box<dyn Error>>;
    async fn list_users(
        &self,
        request: ListUsersRequest,
    ) -> Result<ListUsersResponse, Box<dyn Error>>;
    async fn assign_role(
        &self,
        request: AssignRoleRequest,
    ) -> Result<AssignRoleResponse, Box<dyn Error>>;
    async fn revoke_role(
        &self,
        request: RevokeRoleRequest,
    ) -> Result<RevokeRoleResponse, Box<dyn Error>>;
    async fn check_permission(
        &self,
        request: CheckPermissionRequest,
    ) -> Result<CheckPermissionResponse, Box<dyn Error>>;
    async fn get_user_roles(&self, user_id: i32) -> Result<GetUserRolesResponse, Box<dyn Error>>;
    async fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Result<GetUserPermissionsResponse, Box<dyn Error>>;
}

pub struct UserUseCasesImpl<T: UserService> {
    user_service: T,
}

impl<T: UserService> UserUseCasesImpl<T> {
    pub fn new(user_service: T) -> Self {
        Self { user_service }
    }
}

#[async_trait]
impl<T: UserService> UserUseCases for UserUseCasesImpl<T> {
    async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, Box<dyn Error>> {
        // Hash the password
        let password_hash = self.user_service.hash_password(&request.password).await?;

        let new_user = NewUser {
            username: request.username,
            email: request.email,
            password_hash,
            first_name: request.first_name,
            last_name: request.last_name,
        };

        let user = self.user_service.create_user(new_user).await?;

        Ok(CreateUserResponse { user })
    }

    async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, Box<dyn Error>> {
        let user_info = match (request.id, request.username, request.email) {
            (Some(id), _, _) => self
                .user_service
                .get_user_info(id)
                .await?
                .ok_or("User not found")?,
            (_, Some(username), _) => {
                let user = self
                    .user_service
                    .get_user_by_username(&username)
                    .await?
                    .ok_or("User not found")?;
                self.user_service
                    .get_user_info(user.id.ok_or("Invalid user")?)
                    .await?
                    .ok_or("User info not found")?
            }
            (_, _, Some(email)) => {
                let user = self
                    .user_service
                    .get_user_by_email(&email)
                    .await?
                    .ok_or("User not found")?;
                self.user_service
                    .get_user_info(user.id.ok_or("Invalid user")?)
                    .await?
                    .ok_or("User info not found")?
            }
            _ => {
                return Err(
                    "At least one identifier (id, username, or email) must be provided".into(),
                );
            }
        };

        Ok(GetUserResponse { user_info })
    }

    async fn update_user(
        &self,
        request: UpdateUserRequest,
    ) -> Result<UpdateUserResponse, Box<dyn Error>> {
        let update_user = UpdateUser {
            username: request.username,
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            active: request.active,
        };

        let user = self
            .user_service
            .update_user(request.id, update_user)
            .await?;

        Ok(UpdateUserResponse { user })
    }

    async fn delete_user(
        &self,
        request: DeleteUserRequest,
    ) -> Result<DeleteUserResponse, Box<dyn Error>> {
        let success = self.user_service.delete_user(request.id).await?;

        Ok(DeleteUserResponse { success })
    }

    async fn list_users(
        &self,
        request: ListUsersRequest,
    ) -> Result<ListUsersResponse, Box<dyn Error>> {
        let limit = request.limit.unwrap_or(20);
        let offset = request.offset.unwrap_or(0);

        let users = self.user_service.list_users(limit, offset).await?;

        Ok(ListUsersResponse { users })
    }

    async fn assign_role(
        &self,
        request: AssignRoleRequest,
    ) -> Result<AssignRoleResponse, Box<dyn Error>> {
        let success = self
            .user_service
            .assign_role(request.user_id, request.role_id)
            .await?;

        Ok(AssignRoleResponse { success })
    }

    async fn revoke_role(
        &self,
        request: RevokeRoleRequest,
    ) -> Result<RevokeRoleResponse, Box<dyn Error>> {
        let success = self
            .user_service
            .revoke_role(request.user_id, request.role_id)
            .await?;

        Ok(RevokeRoleResponse { success })
    }

    async fn check_permission(
        &self,
        request: CheckPermissionRequest,
    ) -> Result<CheckPermissionResponse, Box<dyn Error>> {
        let has_permission = self
            .user_service
            .has_permission(request.user_id, &request.resource, &request.action)
            .await?;

        Ok(CheckPermissionResponse { has_permission })
    }

    async fn get_user_roles(&self, user_id: i32) -> Result<GetUserRolesResponse, Box<dyn Error>> {
        let roles = self.user_service.get_user_roles(user_id).await?;

        Ok(GetUserRolesResponse { roles })
    }

    async fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Result<GetUserPermissionsResponse, Box<dyn Error>> {
        let permissions = self.user_service.get_user_permissions(user_id).await?;

        Ok(GetUserPermissionsResponse { permissions })
    }
}
