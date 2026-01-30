use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{delete, insert_into, prelude::*};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::blacklisted_token::{BlacklistedTokenEntity, NewBlacklistedTokenEntity},
        repositories::blacklist_repository::BlacklistRepository,
    },
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad, schema::blacklisted_tokens,
    },
};

pub struct BlacklistPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BlacklistPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BlacklistRepository for BlacklistPostgres {
    async fn add(&self, jti: Uuid, expires_at: DateTime<Utc>) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let new_token = NewBlacklistedTokenEntity { jti, expires_at };

        insert_into(blacklisted_tokens::table)
            .values(new_token)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn is_blacklisted(&self, jti: Uuid) -> Result<bool> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = blacklisted_tokens::table
            .filter(blacklisted_tokens::jti.eq(jti))
            .select(BlacklistedTokenEntity::as_select())
            .first::<BlacklistedTokenEntity>(&mut conn)
            .optional()?;

        Ok(result.is_some())
    }

    async fn cleanup_expired(&self) -> Result<usize> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let now = Utc::now();
        let deleted =
            delete(blacklisted_tokens::table.filter(blacklisted_tokens::expires_at.lt(now)))
                .execute(&mut conn)?;

        Ok(deleted)
    }
}
