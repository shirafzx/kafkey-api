use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::domain::entities::api_key::{ApiKeyEntity, NewApiKeyEntity};
use crate::domain::repositories::api_key_repository::ApiKeyRepository;
use crate::infrastructure::database::postgres::schema::api_keys;

pub struct PostgresApiKeyRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresApiKeyRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApiKeyRepository for PostgresApiKeyRepository {
    async fn create(
        &self,
        tenant_id: Uuid,
        key_hash: String,
        key_prefix: String,
        name: String,
        environment: Option<String>,
    ) -> Result<Uuid> {
        let mut conn = self.pool.get()?;

        let new_key = NewApiKeyEntity {
            tenant_id,
            key_hash,
            key_prefix,
            name,
            environment,
        };

        let api_key: ApiKeyEntity = diesel::insert_into(api_keys::table)
            .values(&new_key)
            .returning(ApiKeyEntity::as_returning())
            .get_result(&mut conn)?;

        Ok(api_key.id)
    }

    async fn find_by_hash(&self, key_hash: String) -> Result<ApiKeyEntity> {
        let mut conn = self.pool.get()?;

        let api_key = api_keys::table
            .filter(api_keys::key_hash.eq(key_hash))
            .filter(api_keys::is_active.eq(true))
            .first::<ApiKeyEntity>(&mut conn)?;

        // Check expiration
        if let Some(expires_at) = api_key.expires_at {
            if chrono::Utc::now() > expires_at {
                return Err(anyhow::anyhow!("API key has expired"));
            }
        }

        Ok(api_key)
    }

    async fn find_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKeyEntity>> {
        let mut conn = self.pool.get()?;

        let api_keys = api_keys::table
            .filter(api_keys::tenant_id.eq(tenant_id))
            .filter(api_keys::is_active.eq(true))
            .load::<ApiKeyEntity>(&mut conn)?;

        Ok(api_keys)
    }

    async fn update_last_used(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(api_keys::table.filter(api_keys::id.eq(id)))
            .set(api_keys::last_used_at.eq(chrono::Utc::now()))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn revoke(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(api_keys::table.filter(api_keys::id.eq(id)))
            .set(api_keys::is_active.eq(false))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::delete(api_keys::table.filter(api_keys::id.eq(id))).execute(&mut conn)?;

        Ok(())
    }
}
