use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::domain::entities::tenant_admin::{NewTenantAdminEntity, TenantAdminEntity};
use crate::domain::repositories::tenant_admin_repository::TenantAdminRepository;
use crate::infrastructure::database::postgres::schema::tenant_admins;

pub struct PostgresTenantAdminRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresTenantAdminRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantAdminRepository for PostgresTenantAdminRepository {
    async fn create(
        &self,
        email: String,
        password_hash: String,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<Uuid> {
        let mut conn = self.pool.get()?;

        let new_admin = NewTenantAdminEntity {
            email,
            password_hash,
            name,
            company_name,
        };

        let admin: TenantAdminEntity = diesel::insert_into(tenant_admins::table)
            .values(&new_admin)
            .returning(TenantAdminEntity::as_returning())
            .get_result(&mut conn)?;

        Ok(admin.id)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<TenantAdminEntity> {
        let mut conn = self.pool.get()?;

        let admin = tenant_admins::table
            .filter(tenant_admins::id.eq(id))
            .first::<TenantAdminEntity>(&mut conn)?;

        Ok(admin)
    }

    async fn find_by_email(&self, email: String) -> Result<TenantAdminEntity> {
        let mut conn = self.pool.get()?;

        let admin = tenant_admins::table
            .filter(tenant_admins::email.eq(email))
            .first::<TenantAdminEntity>(&mut conn)?;

        Ok(admin)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        company_name: Option<String>,
    ) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenant_admins::table.filter(tenant_admins::id.eq(id)))
            .set((
                name.map(|n| tenant_admins::name.eq(n)),
                company_name.map(|c| tenant_admins::company_name.eq(c)),
                tenant_admins::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn verify_email(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenant_admins::table.filter(tenant_admins::id.eq(id)))
            .set((
                tenant_admins::email_verified.eq(true),
                tenant_admins::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenant_admins::table.filter(tenant_admins::id.eq(id)))
            .set((
                tenant_admins::password_hash.eq(password_hash),
                tenant_admins::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn deactivate(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenant_admins::table.filter(tenant_admins::id.eq(id)))
            .set((
                tenant_admins::is_active.eq(false),
                tenant_admins::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}
