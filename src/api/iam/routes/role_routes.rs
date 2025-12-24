use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::api::iam::handlers::role::{
    assign_permission, assign_permission_to_user, create_role, delete_role, get_role,
    get_role_by_name, get_role_permissions, list_roles, revoke_permission, update_role,
};
use crate::api::iam::middleware::auth_middleware::auth_required;

pub fn role_routes<T: Clone + Send + Sync + 'static>(state: Arc<T>) -> Router {
    Router::new()
        // Public endpoints (read-only)
        .route("/", get(list_roles::<T>))
        .route("/:id", get(get_role::<T>))
        .route("/name/:name", get(get_role_by_name::<T>))
        .route("/:id/permissions", get(get_role_permissions::<T>))
        // Protected endpoints
        .route("/", post(create_role::<T>))
        .route("/:id", put(update_role::<T>))
        .route("/:id", delete(delete_role::<T>))
        .route("/:id/permissions", post(assign_permission::<T>))
        .route("/:id/permissions", delete(revoke_permission::<T>))
        .route(
            "/:id/permissions/assign",
            post(assign_permission_to_user::<T>),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_required::<T>,
        ))
        .with_state(state)
}
