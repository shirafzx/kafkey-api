use anyhow::Result;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(
    database_url: &str,
    max_connections: u32,
    min_idle: Option<u32>,
) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(max_connections)
        .min_idle(min_idle)
        .build(manager)?;

    Ok(pool)
}
