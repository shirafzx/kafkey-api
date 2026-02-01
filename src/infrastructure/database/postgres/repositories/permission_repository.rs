use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{insert_into, prelude::*};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::permission::{NewPermissionEntity, PermissionEntity},
        repositories::permission_repository::PermissionRepository,
    },
    infrastructure::database::postgres::{postgres_connection::PgPoolSquad, schema::permissions},
};

pub struct PermissionPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl PermissionPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl PermissionRepository for PermissionPostgres {
    async fn create(&self, new_permission: NewPermissionEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(permissions::table)
            .values(new_permission)
            .returning(permissions::id)
            .get_result::<Uuid>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<PermissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = permissions::table
            .filter(permissions::id.eq(id))
            .select(PermissionEntity::as_select())
            .first::<PermissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_name(&self, name: String) -> Result<PermissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = permissions::table
            .filter(permissions::name.eq(name))
            .select(PermissionEntity::as_select())
            .first::<PermissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_all(&self) -> Result<Vec<PermissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = permissions::table
            .select(PermissionEntity::as_select())
            .load::<PermissionEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update(
        &self,
        permission_id: Uuid,
        name_opt: Option<String>,
        description_opt: Option<String>,
    ) -> Result<()> {
        use crate::infrastructure::database::postgres::schema::permissions::dsl::*;

        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            conn.transaction::<_, anyhow::Error, _>(|conn| {
                if let Some(n) = name_opt {
                    diesel::update(permissions.filter(id.eq(permission_id)))
                        .set(name.eq(n))
                        .execute(conn)?;
                }

                if let Some(desc) = description_opt {
                    diesel::update(permissions.filter(id.eq(permission_id)))
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

    async fn delete(&self, permission_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(permissions::table.filter(permissions::id.eq(permission_id)))
            .execute(&mut conn)?;

        Ok(())
    }

    // Tenant-scoped methods
    async fn find_by_id_scoped(&self, id: Uuid, tenant_id: Uuid) -> Result<PermissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = permissions::table
            .filter(permissions::id.eq(id))
            .filter(permissions::tenant_id.eq(tenant_id))
            .select(PermissionEntity::as_select())
            .first::<PermissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_name_scoped(&self, name: String, tenant_id: Uuid) -> Result<PermissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = permissions::table
            .filter(permissions::name.eq(name))
            .filter(permissions::tenant_id.eq(tenant_id))
            .select(PermissionEntity::as_select())
            .first::<PermissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_all_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<PermissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = permissions::table
            .filter(permissions::tenant_id.eq(tenant_id))
            .select(PermissionEntity::as_select())
            .load::<PermissionEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update_scoped(
        &self,
        permission_id: Uuid,
        tenant_id_val: Uuid,
        name_opt: Option<String>,
        description_opt: Option<String>,
    ) -> Result<()> {
        use crate::infrastructure::database::postgres::schema::permissions::dsl::*;

        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            conn.transaction::<_, anyhow::Error, _>(|conn| {
                if let Some(n) = name_opt {
                    diesel::update(
                        permissions
                            .filter(id.eq(permission_id))
                            .filter(tenant_id.eq(tenant_id_val)),
                    )
                    .set(name.eq(n))
                    .execute(conn)?;
                }

                if let Some(desc) = description_opt {
                    diesel::update(
                        permissions
                            .filter(id.eq(permission_id))
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

    async fn delete_scoped(&self, permission_id: Uuid, tenant_id_val: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(
            permissions::table
                .filter(permissions::id.eq(permission_id))
                .filter(permissions::tenant_id.eq(tenant_id_val)),
        )
        .execute(&mut conn)?;

        Ok(())
    }
}
