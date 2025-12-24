use crate::domain::entities::iam::permission::{
    NewPermission, Permission, PermissionInfo, UpdatePermission,
};
use crate::domain::repositories::iam::PermissionRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::sync::Arc;

use crate::infrastructure::database::postgres::schema::permissions;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

pub struct PermissionRepositoryImpl {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl PermissionRepositoryImpl {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PermissionRepository for PermissionRepositoryImpl {
    async fn create_permission(
        &self,
        new_permission: NewPermission,
    ) -> Result<Permission, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission = diesel::insert_into(permissions::table)
            .values(&new_permission)
            .get_result::<Permission>(&mut conn)
            .await?;

        Ok(permission)
    }

    async fn get_permission_by_id(
        &self,
        permission_id: i32,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission = permissions::table
            .filter(permissions::id.eq(permission_id))
            .first::<Permission>(&mut conn)
            .await;

        match permission {
            Ok(permission) => Ok(Some(permission)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_permission_by_name(
        &self,
        name: &str,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission = permissions::table
            .filter(permissions::name.eq(name))
            .first::<Permission>(&mut conn)
            .await;

        match permission {
            Ok(permission) => Ok(Some(permission)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_permission_by_resource_action(
        &self,
        resource: &str,
        action: &str,
    ) -> Result<Option<Permission>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission = permissions::table
            .filter(
                permissions::resource
                    .eq(resource)
                    .and(permissions::action.eq(action)),
            )
            .first::<Permission>(&mut conn)
            .await;

        match permission {
            Ok(permission) => Ok(Some(permission)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_permission_info(
        &self,
        permission_id: i32,
    ) -> Result<Option<PermissionInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission = permissions::table
            .filter(permissions::id.eq(permission_id))
            .first::<Permission>(&mut conn)
            .await;

        match permission {
            Ok(permission) => {
                let permission_info = PermissionInfo {
                    id: permission.id.unwrap_or(permission_id),
                    name: permission.name,
                    resource: permission.resource,
                    action: permission.action,
                    description: permission.description,
                };
                Ok(Some(permission_info))
            }
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_permission(
        &self,
        permission_id: i32,
        update_permission: UpdatePermission,
    ) -> Result<Permission, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permission =
            diesel::update(permissions::table.filter(permissions::id.eq(permission_id)))
                .set(&update_permission)
                .get_result::<Permission>(&mut conn)
                .await?;

        Ok(permission)
    }

    async fn delete_permission(&self, permission_id: i32) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(permissions::table.filter(permissions::id.eq(permission_id)))
            .execute(&mut conn)
            .await?;

        Ok(result > 0)
    }

    async fn list_permissions(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PermissionInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let permissions = permissions::table
            .limit(limit)
            .offset(offset)
            .load::<Permission>(&mut conn)
            .await?
            .into_iter()
            .map(|permission| PermissionInfo {
                id: permission.id.unwrap_or(0),
                name: permission.name,
                resource: permission.resource,
                action: permission.action,
                description: permission.description,
            })
            .collect();

        Ok(permissions)
    }

    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<Vec<PermissionInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // This is a simplified version - in a real implementation, you would join with role_permissions
        // to get the role's permissions
        let permissions = permissions::table
            .load::<Permission>(&mut conn)
            .await?
            .into_iter()
            .map(|permission| PermissionInfo {
                id: permission.id.unwrap_or(0),
                name: permission.name,
                resource: permission.resource,
                action: permission.action,
                description: permission.description,
            })
            .collect();

        Ok(permissions)
    }
}

// Implement Queryable for Permission
impl Queryable<permissions::SqlType, diesel::pg::Pg> for Permission {
    type Row = (
        Option<i32>,
        String,
        String,
        String,
        Option<String>,
        Option<DateTime<Utc>>,
        Option<DateTime<Utc>>,
    );

    fn build(row: Self::Row) -> Self {
        Permission {
            id: row.0,
            name: row.1,
            resource: row.2,
            action: row.3,
            description: row.4,
            created_at: row.5,
            updated_at: row.6,
        }
    }
}

// Implement Insertable for NewPermission
impl Insertable<permissions::table> for NewPermission {
    type Values = <(
        Option<diesel::dsl::Eq<permissions::columns::id, i32>>,
        diesel::dsl::Eq<permissions::columns::name, String>,
        diesel::dsl::Eq<permissions::columns::resource, String>,
        diesel::dsl::Eq<permissions::columns::action, String>,
        Option<diesel::dsl::Eq<permissions::columns::description, String>>,
    ) as Insertable<permissions::table>>::Values;

    fn values(self) -> Self::Values {
        (
            None,
            self.name,
            self.resource,
            self.action,
            self.description,
        )
            .values()
    }
}

// Implement AsChangeset for UpdatePermission
impl AsChangeset for UpdatePermission {
    type Target = permissions::table;
    type Changeset = <(
        Option<diesel::dsl::Eq<permissions::columns::name, String>>,
        Option<diesel::dsl::Eq<permissions::columns::resource, String>>,
        Option<diesel::dsl::Eq<permissions::columns::action, String>>,
        Option<diesel::dsl::Eq<permissions::columns::description, String>>,
    ) as AsChangeset>::Changeset;

    fn as_changeset(self) -> Self::Changeset {
        (
            self.name.map(|v| permissions::name.eq(v)),
            self.resource.map(|v| permissions::resource.eq(v)),
            self.action.map(|v| permissions::action.eq(v)),
            self.description.map(|v| permissions::description.eq(v)),
        )
            .as_changeset()
    }
}
