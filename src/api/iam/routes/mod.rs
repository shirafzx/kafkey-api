use axum::Router;
use std::sync::Arc;

pub mod auth_routes;
pub mod permission_routes;
pub mod role_routes;
pub mod user_routes;

pub use auth_routes::*;
pub use permission_routes::*;
pub use role_routes::*;
pub use user_routes::*;

// Main IAM router that combines all sub-routers
pub fn iam_router<T: Clone + Send + Sync + 'static>(state: Arc<T>) -> Router {
    Router::new()
        .nest("/auth", auth_routes(state.clone()))
        .nest("/users", user_routes(state.clone()))
        .nest("/roles", role_routes(state.clone()))
        .nest("/permissions", permission_routes(state))
}
