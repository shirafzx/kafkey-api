use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{delete, insert_into, prelude::*, update};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        entities::{
            permission::PermissionEntity,
            role::RoleEntity,
            user::{NewUserEntity, UserEntity},
            user_role::NewUserRoleEntity,
        },
        repositories::user_repository::UserRepository,
    },
    infrastructure::database::postgres::{
        postgres_connection::PgPoolSquad,
        schema::{permissions, role_permissions, roles, user_roles, users},
    },
};

pub struct UserPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl UserPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for UserPostgres {
    async fn create(&self, new_user: NewUserEntity) -> Result<Uuid> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = insert_into(users::table)
            .values(new_user)
            .returning(users::id)
            .get_result::<Uuid>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = users::table
            .filter(users::id.eq(id))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = users::table
            .filter(users::username.eq(username))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_email(&self, email: String) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let result = users::table
            .filter(users::email.eq(email))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }

    async fn update_last_login(&self, id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        update(users::table.filter(users::id.eq(id)))
            .set(users::last_login_at.eq(diesel::dsl::now))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn increment_failed_login(&self, id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        update(users::table.filter(users::id.eq(id)))
            .set(users::failed_login_attempts.eq(users::failed_login_attempts + 1))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn reset_failed_login(&self, id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        update(users::table.filter(users::id.eq(id)))
            .set((
                users::failed_login_attempts.eq(0),
                users::locked_at.eq::<Option<chrono::DateTime<chrono::Utc>>>(None),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn lock_account(&self, id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        update(users::table.filter(users::id.eq(id)))
            .set(users::locked_at.eq(diesel::dsl::now))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let new_assignment = NewUserRoleEntity { user_id, role_id };

        insert_into(user_roles::table)
            .values(new_assignment)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn remove_role(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        delete(
            user_roles::table
                .filter(user_roles::user_id.eq(user_id))
                .filter(user_roles::role_id.eq(role_id)),
        )
        .execute(&mut conn)?;

        Ok(())
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<RoleEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = user_roles::table
            .filter(user_roles::user_id.eq(user_id))
            .inner_join(roles::table)
            .select(RoleEntity::as_select())
            .load::<RoleEntity>(&mut conn)?;

        Ok(results)
    }

    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<PermissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = user_roles::table
            .filter(user_roles::user_id.eq(user_id))
            .inner_join(roles::table)
            .inner_join(role_permissions::table.on(roles::id.eq(role_permissions::role_id)))
            .inner_join(permissions::table.on(role_permissions::permission_id.eq(permissions::id)))
            .select(PermissionEntity::as_select())
            .distinct()
            .load::<PermissionEntity>(&mut conn)?;

        Ok(results)
    }

    async fn update_profile(
        &self,
        user_id: Uuid,
        display_name_opt: Option<String>,
        avatar_image_url_opt: Option<String>,
    ) -> Result<()> {
        self.admin_update(user_id, display_name_opt, avatar_image_url_opt, None, None)
            .await
    }

    async fn admin_update(
        &self,
        user_id: Uuid,
        display_name_opt: Option<String>,
        avatar_image_url_opt: Option<String>,
        is_active_opt: Option<bool>,
        is_verified_opt: Option<bool>,
    ) -> Result<()> {
        use crate::infrastructure::database::postgres::schema::users::dsl::*;

        let pool = Arc::clone(&self.db_pool);

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            conn.transaction::<_, anyhow::Error, _>(|conn| {
                if let Some(name) = display_name_opt {
                    diesel::update(users.filter(id.eq(user_id)))
                        .set(display_name.eq(name))
                        .execute(conn)?;
                }

                if let Some(avatar) = avatar_image_url_opt {
                    diesel::update(users.filter(id.eq(user_id)))
                        .set(avatar_image_url.eq(Some(avatar)))
                        .execute(conn)?;
                }

                if let Some(active) = is_active_opt {
                    diesel::update(users.filter(id.eq(user_id)))
                        .set(is_active.eq(Some(active)))
                        .execute(conn)?;
                }

                if let Some(verified) = is_verified_opt {
                    diesel::update(users.filter(id.eq(user_id)))
                        .set(is_verified.eq(Some(verified)))
                        .execute(conn)?;
                }

                Ok(())
            })?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<UserEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = users::table
            .select(UserEntity::as_select())
            .load::<UserEntity>(&mut conn)?;

        Ok(results)
    }

    async fn find_paginated(&self, limit: i64, offset: i64) -> Result<Vec<UserEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let results = users::table
            .select(UserEntity::as_select())
            .limit(limit)
            .offset(offset)
            .load::<UserEntity>(&mut conn)?;

        Ok(results)
    }

    async fn count(&self) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let total = users::table.count().get_result::<i64>(&mut conn)?;

        Ok(total)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        diesel::delete(users::table.filter(users::id.eq(id))).execute(&mut conn)?;

        Ok(())
    }
}
