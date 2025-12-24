use crate::domain::entities::iam::user::{NewUser, UpdateUser, User, UserInfo};
use crate::domain::repositories::iam::UserRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::sync::Arc;

use crate::infrastructure::database::postgres::schema::users;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

pub struct UserRepositoryImpl {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, new_user: NewUser) -> Result<User, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .await?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user = users::table
            .filter(users::id.eq(user_id))
            .first::<User>(&mut conn)
            .await;

        match user {
            Ok(user) => Ok(Some(user)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user = users::table
            .filter(users::username.eq(username))
            .first::<User>(&mut conn)
            .await;

        match user {
            Ok(user) => Ok(Some(user)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user = users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn)
            .await;

        match user {
            Ok(user) => Ok(Some(user)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_user_info(&self, user_id: i32) -> Result<Option<UserInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with user_roles
        // and roles to get the user's roles
        let user = users::table
            .filter(users::id.eq(user_id))
            .first::<User>(&mut conn)
            .await;

        match user {
            Ok(user) => {
                let user_info = UserInfo {
                    id: user.id.unwrap_or(user_id),
                    username: user.username,
                    email: user.email,
                    first_name: user.first_name,
                    last_name: user.last_name,
                    active: user.active,
                    roles: vec![], // Simplified - would fetch roles in a real implementation
                };
                Ok(Some(user_info))
            }
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_user(
        &self,
        user_id: i32,
        update_user: UpdateUser,
    ) -> Result<User, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let user = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(&update_user)
            .get_result::<User>(&mut conn)
            .await?;

        Ok(user)
    }

    async fn delete_user(&self, user_id: i32) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(users::table.filter(users::id.eq(user_id)))
            .execute(&mut conn)
            .await?;

        Ok(result > 0)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<UserInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with user_roles
        // and roles to get the user's roles
        let users = users::table
            .limit(limit)
            .offset(offset)
            .load::<User>(&mut conn)
            .await?
            .into_iter()
            .map(|user| UserInfo {
                id: user.id.unwrap_or(0),
                username: user.username,
                email: user.email,
                first_name: user.first_name,
                last_name: user.last_name,
                active: user.active,
                roles: vec![], // Simplified - would fetch roles in a real implementation
            })
            .collect();

        Ok(users)
    }

    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        // In a real implementation, you would hash the password and compare with stored hash
        // For now, we'll just fetch the user
        self.get_user_by_username(username).await
    }
}

// Implement Queryable for User
impl Queryable<users::SqlType, diesel::pg::Pg> for User {
    type Row = (
        Option<i32>,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
        bool,
        Option<DateTime<Utc>>,
        Option<DateTime<Utc>>,
    );

    fn build(row: Self::Row) -> Self {
        User {
            id: row.0,
            username: row.1,
            email: row.2,
            password_hash: row.3,
            first_name: row.4,
            last_name: row.5,
            active: row.6,
            created_at: row.7,
            updated_at: row.8,
        }
    }
}

// Implement Insertable for NewUser
impl Insertable<users::table> for NewUser {
    type Values = <(
        Option<diesel::dsl::Eq<users::columns::id, i32>>,
        diesel::dsl::Eq<users::columns::username, String>,
        diesel::dsl::Eq<users::columns::email, String>,
        diesel::dsl::Eq<users::columns::password_hash, String>,
        Option<diesel::dsl::Eq<users::columns::first_name, String>>,
        Option<diesel::dsl::Eq<users::columns::last_name, String>>,
    ) as Insertable<users::table>>::Values;

    fn values(self) -> Self::Values {
        (
            None,
            self.username,
            self.email,
            self.password_hash,
            self.first_name,
            self.last_name,
        )
            .values()
    }
}

// Implement AsChangeset for UpdateUser
impl AsChangeset for UpdateUser {
    type Target = users::table;
    type Changeset = <(
        Option<diesel::dsl::Eq<users::columns::username, String>>,
        Option<diesel::dsl::Eq<users::columns::email, String>>,
        Option<diesel::dsl::Eq<users::columns::first_name, String>>,
        Option<diesel::dsl::Eq<users::columns::last_name, String>>,
        Option<diesel::dsl::Eq<users::columns::active, bool>>,
    ) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        (
            self.username.map(|v| users::username.eq(v)),
            self.email.map(|v| users::email.eq(v)),
            self.first_name.map(|v| users::first_name.eq(v)),
            self.last_name.map(|v| users::last_name.eq(v)),
            self.active.map(|v| users::active.eq(v)),
        )
            .as_changeset()
    }
}
