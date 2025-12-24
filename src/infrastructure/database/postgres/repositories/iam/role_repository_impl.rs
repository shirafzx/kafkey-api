use crate::domain::entities::iam::role::{NewRole, Role, RoleInfo, UpdateRole};
use crate::domain::repositories::iam::RoleRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::sync::Arc;

use crate::infrastructure::database::postgres::schema::roles;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

pub struct RoleRepositoryImpl {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl RoleRepositoryImpl {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
    async fn create_role(&self, new_role: NewRole) -> Result<Role, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role = diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result::<Role>(&mut conn)
            .await?;

        Ok(role)
    }

    async fn get_role_by_id(&self, role_id: i32) -> Result<Option<Role>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role = roles::table
            .filter(roles::id.eq(role_id))
            .first::<Role>(&mut conn)
            .await;

        match role {
            Ok(role) => Ok(Some(role)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_role_by_name(&self, name: &str) -> Result<Option<Role>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role = roles::table
            .filter(roles::name.eq(name))
            .first::<Role>(&mut conn)
            .await;

        match role {
            Ok(role) => Ok(Some(role)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_role_info(&self, role_id: i32) -> Result<Option<RoleInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with role_permissions
        // and permissions to get the role's permissions
        let role = roles::table
            .filter(roles::id.eq(role_id))
            .first::<Role>(&mut conn)
            .await;

        match role {
            Ok(role) => {
                let role_info = RoleInfo {
                    id: role.id.unwrap_or(role_id),
                    name: role.name,
                    description: role.description,
                    permissions: vec![], // Simplified - would fetch permissions in a real implementation
                };
                Ok(Some(role_info))
            }
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_role(
        &self,
        role_id: i32,
        update_role: UpdateRole,
    ) -> Result<Role, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role = diesel::update(roles::table.filter(roles::id.eq(role_id)))
            .set(&update_role)
            .get_result::<Role>(&mut conn)
            .await?;

        Ok(role)
    }

    async fn delete_role(&self, role_id: i32) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(roles::table.filter(roles::id.eq(role_id)))
            .execute(&mut conn)
            .await?;

        Ok(result > 0)
    }

    async fn list_roles(&self, limit: i64, offset: i64) -> Result<Vec<RoleInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with role_permissions
        // and permissions to get the role's permissions
        let roles = roles::table
            .limit(limit)
            .offset(offset)
            .load::<Role>(&mut conn)
            .await?
            .into_iter()
            .map(|role| RoleInfo {
                id: role.id.unwrap_or(0),
                name: role.name,
                description: role.description,
                permissions: vec![], // Simplified - would fetch permissions in a real implementation
            })
            .collect();

        Ok(roles)
    }

    async fn get_user_roles(&self, user_id: i32) -> Result<Vec<RoleInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with user_roles
        // and roles to get the user's roles
        let roles = roles::table
            .load::<Role>(&mut conn)
            .await?
            .into_iter()
            .map(|role| RoleInfo {
                id: role.id.unwrap_or(0),
                name: role.name,
                description: role.description,
                permissions: vec![], // Simplified - would fetch permissions in a real implementation
            })
            .collect();

        Ok(roles)
    }
}

// Implement Queryable for Role
impl Queryable<roles::SqlType, diesel::pg::Pg> for Role {
    type Row = (
        Option<i32>,
        String,
        Option<String>,
        Option<DateTime<Utc>>,
        Option<DateTime<Utc>>,
    );

    fn build(row: Self::Row) -> Self {
        Role {
            id: row.0,
            name: row.1,
            description: row.2,
            created_at: row.3,
            updated_at: row.4,
        }
    }
}

// Implement Insertable for NewRole
impl Insertable<roles::table> for NewRole {
    type Values = <(
        Option<diesel::dsl::Eq<roles::columns::id, i32>>,
        diesel::dsl::Eq<roles::columns::name, String>,
        Option<diesel::dsl::Eq<roles::columns::description, String>>,
    ) as Insertable<roles::table>>::Values;

    fn values(self) -> Self::Values {
        (None, self.name, self.description).values()
    }
}

// Implement AsChangeset for UpdateRole
impl AsChangeset for UpdateRole {
    type Target = roles::table;
    type Changeset = <(
        Option<diesel::dsl::Eq<roles::columns::name, String>>,
        Option<diesel::dsl::Eq<roles::columns::description, String>>,
    ) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        (
            self.name.map(|v| roles::name.eq(v)),
            self.description.map(|v| roles::description.eq(v)),
        )
            .as_changeset()
    }
}
