use crate::domain::entities::iam::role_permission::{
    NewRolePermission, RolePermission, RolePermissionInfo,
};
use crate::domain::repositories::iam::RolePermissionRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::sync::Arc;

use crate::infrastructure::database::postgres::schema::{permissions, role_permissions, roles};
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::Pool};

pub struct RolePermissionRepositoryImpl {
    pool: Arc<Pool<AsyncPgConnection>>,
}

impl RolePermissionRepositoryImpl {
    pub fn new(pool: Arc<Pool<AsyncPgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RolePermissionRepository for RolePermissionRepositoryImpl {
    async fn assign_permission_to_role(
        &self,
        new_role_permission: NewRolePermission,
    ) -> Result<RolePermission, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role_permission = diesel::insert_into(role_permissions::table)
            .values(&new_role_permission)
            .get_result::<RolePermission>(&mut conn)
            .await?;

        Ok(role_permission)
    }

    async fn remove_permission_from_role(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result = diesel::delete(
            role_permissions::table.filter(
                role_permissions::role_id
                    .eq(role_id)
                    .and(role_permissions::permission_id.eq(permission_id)),
            ),
        )
        .execute(&mut conn)
        .await?;

        Ok(result > 0)
    }

    async fn get_role_permission(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<Option<RolePermission>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let role_permission = role_permissions::table
            .filter(
                role_permissions::role_id
                    .eq(role_id)
                    .and(role_permissions::permission_id.eq(permission_id)),
            )
            .first::<RolePermission>(&mut conn)
            .await;

        match role_permission {
            Ok(role_permission) => Ok(Some(role_permission)),
            Err(DieselError::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Result<Vec<RolePermissionInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // Join role_permissions with roles and permissions to get full info
        let query = role_permissions::table
            .filter(role_permissions::role_id.eq(role_id))
            .inner_join(roles::table.on(role_permissions::role_id.eq(roles::id)))
            .inner_join(permissions::table.on(role_permissions::permission_id.eq(permissions::id)))
            .select((
                role_permissions::id,
                role_permissions::role_id,
                role_permissions::permission_id,
                roles::name,
                permissions::name,
                permissions::resource,
                permissions::action,
            ));

        let results = query
            .load::<(i32, i32, i32, String, String, String, String)>(&mut conn)
            .await?;

        let role_permissions = results
            .into_iter()
            .map(
                |(id, role_id, permission_id, role_name, permission_name, resource, action)| {
                    RolePermissionInfo {
                        id,
                        role_id,
                        permission_id,
                        role_name,
                        permission_name,
                        resource,
                        action,
                    }
                },
            )
            .collect();

        Ok(role_permissions)
    }

    async fn get_permission_roles(
        &self,
        permission_id: i32,
    ) -> Result<Vec<RolePermissionInfo>, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        // Join role_permissions with roles and permissions to get full info
        let query = role_permissions::table
            .filter(role_permissions::permission_id.eq(permission_id))
            .inner_join(roles::table.on(role_permissions::role_id.eq(roles::id)))
            .inner_join(permissions::table.on(role_permissions::permission_id.eq(permissions::id)))
            .select((
                role_permissions::id,
                role_permissions::role_id,
                role_permissions::permission_id,
                roles::name,
                permissions::name,
                permissions::resource,
                permissions::action,
            ));

        let results = query
            .load::<(i32, i32, i32, String, String, String, String)>(&mut conn)
            .await?;

        let role_permissions = results
            .into_iter()
            .map(
                |(id, role_id, permission_id, role_name, permission_name, resource, action)| {
                    RolePermissionInfo {
                        id,
                        role_id,
                        permission_id,
                        role_name,
                        permission_name,
                        resource,
                        action,
                    }
                },
            )
            .collect();

        Ok(role_permissions)
    }

    async fn remove_all_role_permissions(&self, role_id: i32) -> Result<bool, Box<dyn Error>> {
        let mut conn = self.pool.get().await?;

        let result =
            diesel::delete(role_permissions::table.filter(role_permissions::role_id.eq(role_id)))
                .execute(&mut conn)
                .await?;

        Ok(result > 0)
    }
}

// Implement Queryable for RolePermission
impl Queryable<role_permissions::SqlType, diesel::pg::Pg> for RolePermission {
    type Row = (Option<i32>, i32, i32, Option<DateTime<Utc>>);

    fn build(row: Self::Row) -> Self {
        RolePermission {
            id: row.0,
            role_id: row.1,
            permission_id: row.2,
            created_at: row.3,
        }
    }
}

// Implement Insertable for NewRolePermission
impl Insertable<role_permissions::table> for NewRolePermission {
    type Values = <(
        Option<diesel::dsl::Eq<role_permissions::columns::id, i32>>,
        diesel::dsl::Eq<role_permissions::columns::role_id, i32>,
        diesel::dsl::Eq<role_permissions::columns::permission_id, i32>,
    ) as Insertable<role_permissions::table>>::Values;

    fn values(self) -> Self::Values {
        (None, self.role_id, self.permission_id).values()
    }
}
