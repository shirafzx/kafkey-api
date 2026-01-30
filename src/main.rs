use std::sync::Arc;

use kafkey_api::{
    api::axum_http::axum_router, config::config_loader,
    infrastructure::database::postgres::postgres_connection,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Arc::new(config_loader::load()?);

    // Get database connection pool
    let db_pool = Arc::new(postgres_connection::establish_connection(
        &config.database.url,
    )?);

    // Start the server
    axum_router::start(config, db_pool).await;

    Ok(())
}
