use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{
    permission::PermissionEntity,
    role::{NewRoleEntity, RoleEntity},
};

#[async_trait]
pub trait RoleRepository {
    async fn create(&self, new_role: NewRoleEntity) -> Result<Uuid>;
    async fn find_by_id(&self, id: Uuid) -> Result<RoleEntity>;
    async fn find_by_name(&self, name: String) -> Result<RoleEntity>;
    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
    async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()>;
    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<PermissionEntity>>;
    async fn find_all(&self) -> Result<Vec<RoleEntity>>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
