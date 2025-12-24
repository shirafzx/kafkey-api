use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;

use crate::api::iam::handlers::auth::{login, logout, refresh_token, register, validate_token};

pub fn auth_routes<T: Clone + Send + Sync + 'static>(state: Arc<T>) -> Router {
    Router::new()
        .route("/login", post(login::<T>))
        .route("/register", post(register::<T>))
        .route("/refresh", post(refresh_token::<T>))
        .route("/logout", post(logout::<T>))
        .route("/validate", post(validate_token::<T>))
        .with_state(state)
}
