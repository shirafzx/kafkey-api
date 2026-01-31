use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::use_cases::audit::AuditUseCases;
use crate::domain::repositories::audit_repository::AuditRepository;
use crate::domain::{
    entities::permission::PermissionEntity,
    repositories::permission_repository::PermissionRepository,
};

pub struct PermissionUseCases<P, AR>
where
    P: PermissionRepository + Send + Sync,
    AR: AuditRepository + Send + Sync,
{
    permission_repository: Arc<P>,
    audit_use_case: Arc<AuditUseCases<AR>>,
}

impl<P, AR> PermissionUseCases<P, AR>
where
    P: PermissionRepository + Send + Sync,
    AR: AuditRepository + Send + Sync,
{
    pub fn new(permission_repository: Arc<P>, audit_use_case: Arc<AuditUseCases<AR>>) -> Self {
        Self {
            permission_repository,
            audit_use_case,
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

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_PERMISSION_CREATED",
                Some(permission_id),
                "permission",
                "create",
                serde_json::json!({ "name": name }),
            )
            .await
            .ok();

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

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_PERMISSION_UPDATED",
                Some(id),
                "permission",
                "update",
                serde_json::json!({ "name": name }),
            )
            .await
            .ok();

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

        self.audit_use_case
            .log(
                actor_id,
                "AUDIT_PERMISSION_DELETED",
                Some(id),
                "permission",
                "delete",
                serde_json::json!({}),
            )
            .await
            .ok();

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
