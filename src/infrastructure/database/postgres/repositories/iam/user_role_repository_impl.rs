use crate::domain::entities::iam::user_role::{NewUserRole, UserRole, UserRoleInfo};
use crate::domain::repositories::iam::UserRoleRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::sync::Arc;

use crate::infrastructure::database::postgres::schema::{roles, user_roles, users};
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

pub struct UserRoleRepositoryImpl {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserRoleRepositoryImpl {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRoleRepository for UserRoleRepositoryImpl {
    async fn assign_role_to_user(
        &self,
        new_user_role: NewUserRole,
    ) -> Result<UserRole, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user_role = diesel::insert_into(user_roles::table)
            .values(&new_user_role)
            .get_result::<UserRole>(&mut conn)
            .await?;

        Ok(user_role)
    }

    async fn remove_role_from_user(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(
            user_roles::table.filter(
                user_roles::user_id
                    .eq(user_id)
                    .and(user_roles::role_id.eq(role_id)),
            ),
        )
        .execute(&mut conn)
        .await?;

        Ok(result > 0)
    }

    async fn get_user_role(
        &self,
        user_id: i32,
        role_id: i32,
    ) -> Result<Option<UserRole>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user_role = user_roles::table
            .filter(
                user_roles::user_id
                    .eq(user_id)
                    .and(user_roles::role_id.eq(role_id)),
            )
            .first::<UserRole>(&mut conn)
            .await;

        match user_role {
            Ok(user_role) => Ok(Some(user_role)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_roles(&self, user_id: i32) -> Result<Vec<UserRoleInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // In a real implementation, you would join user_roles with users and roles
        // to get the user_role_info with username and role_name
        let query = user_roles::table
            .filter(user_roles::user_id.eq(user_id))
            .inner_join(users::table.on(user_roles::user_id.eq(users::id)))
            .inner_join(roles::table.on(user_roles::role_id.eq(roles::id)))
            .select((
                user_roles::id,
                user_roles::user_id,
                user_roles::role_id,
                users::username,
                roles::name,
            ));

        let results = query
            .load::<(i32, i32, i32, String, String)>(&mut conn)
            .await?;

        let user_roles = results
            .into_iter()
            .map(|(id, user_id, role_id, username, role_name)| UserRoleInfo {
                id,
                user_id,
                role_id,
                username,
                role_name,
            })
            .collect();

        Ok(user_roles)
    }

    async fn get_role_users(&self, role_id: i32) -> Result<Vec<UserRoleInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // In a real implementation, you would join user_roles with users and roles
        // to get the user_role_info with username and role_name
        let query = user_roles::table
            .filter(user_roles::role_id.eq(role_id))
            .inner_join(users::table.on(user_roles::user_id.eq(users::id)))
            .inner_join(roles::table.on(user_roles::role_id.eq(roles::id)))
            .select((
                user_roles::id,
                user_roles::user_id,
                user_roles::role_id,
                users::username,
                roles::name,
            ));

        let results = query
            .load::<(i32, i32, i32, String, String)>(&mut conn)
            .await?;

        let user_roles = results
            .into_iter()
            .map(|(id, user_id, role_id, username, role_name)| UserRoleInfo {
                id,
                user_id,
                role_id,
                username,
                role_name,
            })
            .collect();

        Ok(user_roles)
    }

    async fn remove_all_user_roles(&self, user_id: i32) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(user_roles::table.filter(user_roles::user_id.eq(user_id)))
            .execute(&mut conn)
            .await?;

        Ok(result > 0)
    }
}

// Implement Queryable for UserRole
impl Queryable<user_roles::SqlType, diesel::pg::Pg> for UserRole {
    type Row = (Option<i32>, i32, i32, Option<DateTime<Utc>>);

    fn build(row: Self::Row) -> Self {
        UserRole {
            id: row.0,
            user_id: row.1,
            role_id: row.2,
            created_at: row.3,
        }
    }
}

// Implement Insertable for NewUserRole
impl Insertable<user_roles::table> for NewUserRole {
    type Values = <(
        Option<diesel::dsl::Eq<user_roles::columns::id, i32>>,
        diesel::dsl::Eq<user_roles::columns::user_id, i32>,
        diesel::dsl::Eq<user_roles::columns::role_id, i32>,
    ) as Insertable<user_roles::table>>::Values;

    fn values(self) -> Self::Values {
        (None, self.user_id, self.role_id).values()
    }
}
