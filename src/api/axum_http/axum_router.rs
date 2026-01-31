use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::http::Method;
use axum::{Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::api::axum_http::{default_routers, middleware, routers};
use crate::application::use_cases::audit::AuditUseCases;
use crate::config::{config_loader, config_model::DotEnvyConfig};
use crate::domain::repositories::blacklist_repository::BlacklistRepository;
use crate::infrastructure::database::mongodb::repositories::audit_repository::AuditMongodb;
use crate::infrastructure::database::postgres::postgres_connection::PgPoolSquad;
use crate::services::jwt_service::JwtService;

pub async fn start(
    config: Arc<DotEnvyConfig>,
    db_pool: Arc<PgPoolSquad>,
    mongodb_client: mongodb::Client,
) {
    // Initialize JWT service
    let auth_secrets = config_loader::get_auth_secret_env().expect("Failed to load auth secrets");
    let jwt_service = Arc::new(JwtService::new(
        auth_secrets.secret,
        auth_secrets.refresh_secret,
    ));

    // Initialize Blacklist repository
    let blacklist_repository = Arc::new(
        crate::infrastructure::database::postgres::repositories::blacklist_repository::BlacklistPostgres::new(
            Arc::clone(&db_pool),
        ),
    );

    // Spawn background task for token blacklist cleanup
    let cleanup_repo = Arc::clone(&blacklist_repository);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Every 1 hour
        loop {
            interval.tick().await;
            info!("Running background cleanup for expired blacklisted tokens...");
            match cleanup_repo.cleanup_expired().await {
                Ok(deleted) => info!("Cleaned up {} expired tokens", deleted),
                Err(e) => tracing::error!("Failed to cleanup expired tokens: {}", e),
            }
        }
    });

    // Initialize Rate Limit setup
    let rate_limit_config = middleware::rate_limit_middleware::RateLimitConfig::default();
    let rate_limit_state = middleware::rate_limit_middleware::RateLimitState::new();

    // Initialize Prometheus metrics
    let (prometheus_layer, metric_handle) = axum_prometheus::PrometheusMetricLayerBuilder::new()
        .with_ignore_patterns(&["/metrics", "/health-check"])
        .with_default_metrics()
        .build_pair();

    // Initialize MongoDB Audit repository and use cases
    let audit_repository = Arc::new(AuditMongodb::new(&mongodb_client));
    let audit_use_cases = Arc::new(AuditUseCases::new(Arc::clone(&audit_repository)));

    let app = Router::new()
        .fallback(default_routers::not_found)
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .merge(routers::auth::routes(
            Arc::clone(&db_pool),
            Arc::clone(&jwt_service),
            Arc::clone(&audit_use_cases),
        ))
        .merge(
            routers::users::routes(Arc::clone(&db_pool), Arc::clone(&audit_use_cases))
                .merge(routers::roles::routes(
                    Arc::clone(&db_pool),
                    Arc::clone(&audit_use_cases),
                ))
                .merge(routers::permissions::routes(
                    Arc::clone(&db_pool),
                    Arc::clone(&audit_use_cases),
                ))
                .layer(axum::middleware::from_fn(move |req, next| {
                    let jwt_service = Arc::clone(&jwt_service);
                    let blacklist_repository = Arc::clone(&blacklist_repository);
                    async move {
                        middleware::auth_middleware(jwt_service, blacklist_repository, req, next)
                            .await
                    }
                })),
        )
        .route("/health-check", get(default_routers::health_check))
        .layer(prometheus_layer)
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .layer(axum::middleware::from_fn(middleware::csrf_middleware))
        .layer(axum::middleware::from_fn(move |req, next| {
            let state = Arc::clone(&rate_limit_state);
            async move {
                middleware::rate_limit_middleware::rate_limit_middleware(
                    rate_limit_config,
                    state,
                    req,
                    next,
                )
                .await
            }
        }))
        .layer(axum::middleware::from_fn(
            middleware::request_id::request_id_middleware,
        ))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.server.timeout),
        ))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024)
                .try_into()
                .expect("body limit too large"),
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server is running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C, signal"),
        _ = terminate => info!("Received terminate, signal"),
    }
}
