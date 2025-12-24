use std::sync::Arc;
use std::time::Duration;

use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub async fn start() {
    // Create a regular axum app.
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/slow", get(|| sleep(Duration::from_secs(5))))
        .route("/forever", get(std::future::pending::<()>))
        // Add IAM routes
        .nest(
            "/api/v1/iam",
            // Simplified - in a real implementation, you would inject actual use cases
            Router::new()
                .route(
                    "/",
                    get(|| async { "IAM Service - Role Based Access Control" }),
                )
                .route(
                    "/health",
                    get(|| async {
                        {
                            "healthy"
                        }
                    }),
                ),
        )
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    // Create a `TcpListener` using tokio.
    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    // Run server with graceful shutdown
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
    let terminate = std::future::pending::<()>;

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
