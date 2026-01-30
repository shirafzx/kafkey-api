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

    async fn list_all(&self) -> Result<Vec<PermissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = permissions::table
            .select(PermissionEntity::as_select())
            .load::<PermissionEntity>(&mut conn)?;

        Ok(results)
    }
}
