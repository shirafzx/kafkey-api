use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};

pub struct PermissionUseCases<P>
where
    P: PermissionRepository + Send + Sync,
{
    permission_repository: Arc<P>,
}

impl<P> PermissionUseCases<P>
where
    P: PermissionRepository + Send + Sync,
{
    pub fn new(permission_repository: Arc<P>) -> Self {
        Self {
            permission_repository,
        }
    }

    pub async fn create_permission(
        &self,
        name: String,
        resource: String,
        action: String,
        description: Option<String>,
    ) -> Result<Uuid> {
        let new_permission = crate::domain::entities::permission::NewPermissionEntity {
            name,
            resource,
            action,
            description,
        };
        self.permission_repository.create(new_permission).await
    }

    pub async fn get_permission_by_id(&self, id: Uuid) -> Result<PermissionEntity> {
        self.permission_repository.find_by_id(id).await
    }

    pub async fn list_permissions(&self) -> Result<Vec<PermissionEntity>> {
        self.permission_repository.find_all().await
    }

    pub async fn update_permission(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        self.permission_repository
            .update(id, name, description)
            .await
    }

    pub async fn delete_permission(&self, id: Uuid) -> Result<()> {
        self.permission_repository.delete(id).await
    }
}
