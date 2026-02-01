use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{delete, insert_into, prelude::*};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::{
            permission::PermissionEntity,
            role::{NewRoleEntity, RoleEntity},
            role_permission::NewRolePermissionEntity,
        },
        repositories::role_repository::RoleRepository,
    },
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        schema::{permissions, role_permissions, roles},
    },
};

pub struct RolePostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl RolePostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl RoleRepository for RolePostgres {
    async fn create(&self, new_role: NewRoleEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(roles::table)
            .values(new_role)
            .returning(roles::id)
            .get_result::<Uuid>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<RoleEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = roles::table
            .filter(roles::id.eq(id))
            .select(RoleEntity::as_select())
            .first::<RoleEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_name(&self, name: String) -> Result<RoleEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = roles::table
            .filter(roles::name.eq(name))
            .select(RoleEntity::as_select())
            .first::<RoleEntity>(&mut conn)?;

        Ok(result)
    }

    async fn assign_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let new_assignment = NewRolePermissionEntity {
            role_id,
            permission_id,
        };

        insert_into(role_permissions::table)
            .values(new_assignment)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn remove_permission(&self, role_id: Uuid, permission_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(
            role_permissions::table
                .filter(role_permissions::role_id.eq(role_id))
                .filter(role_permissions::permission_id.eq(permission_id)),
        )
        .execute(&mut conn)?;

        Ok(())
    }

    async fn get_permissions(&self, role_id: Uuid) -> Result<Vec<PermissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = role_permissions::table
            .filter(role_permissions::role_id.eq(role_id))
            .inner_join(permissions::table)
            .select(PermissionEntity::as_select())
            .load::<PermissionEntity>(&mut conn)?;

        Ok(results)
    }

    async fn find_all(&self) -> Result<Vec<RoleEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = roles::table
            .select(RoleEntity::as_select())
            .load::<RoleEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update(
        &self,
        role_id: Uuid,
        name_opt: Option<String>,
        description_opt: Option<String>,
    ) -> Result<()> {
        use crate::infrastructure::database::postgres::schema::roles::dsl::*;

        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            conn.transaction::<_, anyhow::Error, _>(|conn| {
                if let Some(n) = name_opt {
                    diesel::update(roles.filter(id.eq(role_id)))
                        .set(name.eq(n))
                        .execute(conn)?;
                }

                if let Some(desc) = description_opt {
                    diesel::update(roles.filter(id.eq(role_id)))
                        .set(description.eq(Some(desc)))
                        .execute(conn)?;
                }

                Ok(())
            })?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn delete(&self, role_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(roles::table.filter(roles::id.eq(role_id))).execute(&mut conn)?;

        Ok(())
    }

    // Tenant-scoped methods
    async fn find_by_id_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<RoleEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = roles::table
            .filter(roles::id.eq(id))
            .filter(roles::tenant_id.eq(tenant_id))
            .select(RoleEntity::as_select())
            .first::<RoleEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_name_scoped(&self, name: String, tenant_id: Uuid) -> Result<RoleEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = roles::table
            .filter(roles::name.eq(name))
            .filter(roles::tenant_id.eq(tenant_id))
            .select(RoleEntity::as_select())
            .first::<RoleEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_all_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<RoleEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = roles::table
            .filter(roles::tenant_id.eq(tenant_id))
            .select(RoleEntity::as_select())
            .load::<RoleEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update_scoped(
        &self,
        role_id: Uuid,
        tenant_id_val: Uuid,
        name_opt: Option<String>,
        description_opt: Option<String>,
    ) -> Result<()> {
        use crate::infrastructure::database::postgres::schema::roles::dsl::*;

        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            // Verify tenant ownership implicitly by adding filter
            conn.transaction::<_, anyhow::Error, _>(|conn| {
                if let Some(n) = name_opt {
                    diesel::update(
                        roles
                            .filter(id.eq(role_id))
                            .filter(tenant_id.eq(tenant_id_val)),
                    )
                    .set(name.eq(n))
                    .execute(conn)?;
                }

                if let Some(desc) = description_opt {
                    diesel::update(
                        roles
                            .filter(id.eq(role_id))
                            .filter(tenant_id.eq(tenant_id_val)),
                    )
                    .set(description.eq(Some(desc)))
                    .execute(conn)?;
                }

                Ok(())
            })?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn delete_scoped(&self, role_id: Uuid, tenant_id_val: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(
            roles::table
                .filter(roles::id.eq(role_id))
                .filter(roles::tenant_id.eq(tenant_id_val)),
        )
        .execute(&mut conn)?;

        Ok(())
    }
}
