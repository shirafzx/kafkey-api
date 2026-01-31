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
        actor_id: Uuid,
        name: String,
        resource: String,
        action: String,
        description: Option<String>,
    ) -> Result<Uuid> {
        let new_permission = crate::domain::entities::permission::NewPermissionEntity {
            name: name.clone(),
            resource,
            action,
            description,
        };
        let permission_id = self.permission_repository.create(new_permission).await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_PERMISSION_CREATED",
            actor_id = %actor_id,
            target_id = %permission_id,
            permission_name = %name,
            "Administrative action: Permission created"
        );

        Ok(permission_id)
    }

    pub async fn get_permission_by_id(&self, id: Uuid) -> Result<PermissionEntity> {
        self.permission_repository.find_by_id(id).await
    }

    pub async fn list_permissions(&self) -> Result<Vec<PermissionEntity>> {
        self.permission_repository.find_all().await
    }

    pub async fn update_permission(
        &self,
        actor_id: Uuid,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        self.permission_repository
            .update(id, name.clone(), description.clone())
            .await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_PERMISSION_UPDATED",
            actor_id = %actor_id,
            target_id = %id,
            permission_name = ?name,
            "Administrative action: Permission updated"
        );

        Ok(())
    }

    pub async fn delete_permission(&self, actor_id: Uuid, id: Uuid) -> Result<()> {
        self.permission_repository.delete(id).await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_PERMISSION_DELETED",
            actor_id = %actor_id,
            target_id = %id,
            "Administrative action: Permission deleted"
        );

        Ok(())
    }
}
