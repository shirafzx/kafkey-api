use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::domain::entities::tenant::{NewTenantEntity, TenantEntity};
use crate::domain::repositories::tenant_repository::TenantRepository;
use crate::infrastructure::database::postgres::schema::tenants;

pub struct PostgresTenantRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresTenantRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantRepository for PostgresTenantRepository {
    async fn create(
        &self,
        owner_id: Uuid,
        name: String,
        slug: String,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<Uuid> {
        let mut conn = self.pool.get()?;

        let new_tenant = NewTenantEntity {
            owner_id,
            name,
            slug,
            domain,
            logo_url,
        };

        let tenant: TenantEntity = diesel::insert_into(tenants::table)
            .values(&new_tenant)
            .returning(TenantEntity::as_returning())
            .get_result(&mut conn)?;

        Ok(tenant.id)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<TenantEntity> {
        let mut conn = self.pool.get()?;

        let tenant = tenants::table
            .filter(tenants::id.eq(id))
            .filter(tenants::is_active.eq(true))
            .first::<TenantEntity>(&mut conn)?;

        Ok(tenant)
    }

    async fn find_by_slug(&self, slug: String) -> Result<TenantEntity> {
        let mut conn = self.pool.get()?;

        let tenant = tenants::table
            .filter(tenants::slug.eq(slug))
            .filter(tenants::is_active.eq(true))
            .first::<TenantEntity>(&mut conn)?;

        Ok(tenant)
    }

    async fn find_by_owner(&self, owner_id: Uuid) -> Result<Vec<TenantEntity>> {
        let mut conn = self.pool.get()?;

        let tenants = tenants::table
            .filter(tenants::owner_id.eq(owner_id))
            .filter(tenants::is_active.eq(true))
            .load::<TenantEntity>(&mut conn)?;

        Ok(tenants)
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        domain: Option<String>,
        logo_url: Option<String>,
    ) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenants::table.filter(tenants::id.eq(id)))
            .set((
                name.map(|n| tenants::name.eq(n)),
                domain.map(|d| tenants::domain.eq(d)),
                logo_url.map(|l| tenants::logo_url.eq(l)),
                tenants::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn update_plan_tier(&self, id: Uuid, plan_tier: String, max_users: i32) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenants::table.filter(tenants::id.eq(id)))
            .set((
                tenants::plan_tier.eq(plan_tier),
                tenants::max_users.eq(max_users),
                tenants::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn deactivate(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::update(tenants::table.filter(tenants::id.eq(id)))
            .set((
                tenants::is_active.eq(false),
                tenants::updated_at.eq(chrono::Utc::now()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.get()?;

        diesel::delete(tenants::table.filter(tenants::id.eq(id))).execute(&mut conn)?;

        Ok(())
    }
}
