use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::permission::{NewPermissionEntity, PermissionEntity};

#[async_trait]
pub trait PermissionRepository {
    async fn create(&self, new_permission: NewPermissionEntity) -> Result<Uuid>;
    async fn find_by_id(&self, id: Uuid) -> Result<PermissionEntity>;
    async fn find_by_name(&self, name: String) -> Result<PermissionEntity>;
    async fn find_all(&self) -> Result<Vec<PermissionEntity>>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
