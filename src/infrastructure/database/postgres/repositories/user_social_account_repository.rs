use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use uuid::Uuid;

use crate::domain::entities::user_social_account::{
    NewUserSocialAccountEntity, UpdateUserSocialAccountEntity, UserSocialAccountEntity,
};
use crate::domain::repositories::user_social_account_repository::UserSocialAccountRepository;
use crate::infrastructure::database::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::database::postgres::schema::user_social_accounts;
use std::sync::Arc;

pub struct UserSocialAccountPostgres {
    pool: Arc<PgPoolSquad>,
}

impl UserSocialAccountPostgres {
    pub fn new(pool: Arc<PgPoolSquad>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserSocialAccountRepository for UserSocialAccountPostgres {
    async fn create(
        &self,
        new_account: NewUserSocialAccountEntity,
    ) -> Result<UserSocialAccountEntity> {
        let mut conn = self.pool.get()?;

        let account = diesel::insert_into(user_social_accounts::table)
            .values(&new_account)
            .get_result(&mut conn)?;

        Ok(account)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserSocialAccountEntity>> {
        let mut conn = self.pool.get()?;

        let account = user_social_accounts::table
            .find(id)
            .first(&mut conn)
            .optional()?;

        Ok(account)
    }

    async fn find_by_user_and_provider(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<Option<UserSocialAccountEntity>> {
        let mut conn = self.pool.get()?;

        let account = user_social_accounts::table
            .filter(user_social_accounts::user_id.eq(user_id))
            .filter(user_social_accounts::provider.eq(provider))
            .first(&mut conn)
            .optional()?;

        Ok(account)
    }

    async fn find_by_provider_user(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserSocialAccountEntity>> {
        let mut conn = self.pool.get()?;

        let account = user_social_accounts::table
            .filter(user_social_accounts::provider.eq(provider))
            .filter(user_social_accounts::provider_user_id.eq(provider_user_id))
            .first(&mut conn)
            .optional()?;

        Ok(account)
    }

    async fn find_all_by_user(&self, user_id: Uuid) -> Result<Vec<UserSocialAccountEntity>> {
        let mut conn = self.pool.get()?;

        let accounts = user_social_accounts::table
            .filter(user_social_accounts::user_id.eq(user_id))
            .load(&mut conn)?;

        Ok(accounts)
    }

    async fn update(
        &self,
        id: Uuid,
        update: UpdateUserSocialAccountEntity,
    ) -> Result<UserSocialAccountEntity> {
        let mut conn = self.pool.get()?;

        let account = diesel::update(user_social_accounts::table.find(id))
            .set(&update)
            .get_result(&mut conn)?;

        Ok(account)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::delete(user_social_accounts::table.find(id)).execute(&mut conn)?;

        Ok(())
    }
}
