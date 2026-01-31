use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{
    entities::{permission::PermissionEntity, role::RoleEntity},
    repositories::role_repository::RoleRepository,
};

pub struct RoleUseCases<R>
where
    R: RoleRepository + Send + Sync,
{
    role_repository: Arc<R>,
}

impl<R> RoleUseCases<R>
where
    R: RoleRepository + Send + Sync,
{
    pub fn new(role_repository: Arc<R>) -> Self {
        Self { role_repository }
    }

    pub async fn create_role(
        &self,
        actor_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Result<Uuid> {
        let new_role = crate::domain::entities::role::NewRoleEntity {
            name: name.clone(),
            description: description.clone(),
        };
        let role_id = self.role_repository.create(new_role).await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_ROLE_CREATED",
            actor_id = %actor_id,
            target_id = %role_id,
            role_name = %name,
            "Administrative action: Role created"
        );

        Ok(role_id)
    }

    pub async fn get_role_by_id(&self, id: Uuid) -> Result<RoleEntity> {
        self.role_repository.find_by_id(id).await
    }

    pub async fn list_roles(&self) -> Result<Vec<RoleEntity>> {
        self.role_repository.find_all().await
    }

    pub async fn update_role(
        &self,
        actor_id: Uuid,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        self.role_repository
            .update(id, name.clone(), description.clone())
            .await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_ROLE_UPDATED",
            actor_id = %actor_id,
            target_id = %id,
            role_name = ?name,
            "Administrative action: Role updated"
        );

        Ok(())
    }

    pub async fn delete_role(&self, actor_id: Uuid, id: Uuid) -> Result<()> {
        self.role_repository.delete(id).await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_ROLE_DELETED",
            actor_id = %actor_id,
            target_id = %id,
            "Administrative action: Role deleted"
        );

        Ok(())
    }

    pub async fn assign_permission(
        &self,
        actor_id: Uuid,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<()> {
        self.role_repository
            .assign_permission(role_id, permission_id)
            .await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_ROLE_PERMISSION_ASSIGNED",
            actor_id = %actor_id,
            target_id = %role_id,
            permission_id = %permission_id,
            "Administrative action: Permission assigned to role"
        );

        Ok(())
    }

    pub async fn remove_permission(
        &self,
        actor_id: Uuid,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<()> {
        self.role_repository
            .remove_permission(role_id, permission_id)
            .await?;

        tracing::info!(
            audit = true,
            event = "AUDIT_ROLE_PERMISSION_REMOVED",
            actor_id = %actor_id,
            target_id = %role_id,
            permission_id = %permission_id,
            "Administrative action: Permission removed from role"
        );

        Ok(())
    }

    pub async fn get_role_permissions(&self, role_id: Uuid) -> Result<Vec<PermissionEntity>> {
        self.role_repository.get_permissions(role_id).await
    }
}
