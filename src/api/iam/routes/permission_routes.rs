use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::api::iam::handlers::permission::{
    check_permission, create_permission, delete_permission, get_actions_for_resource,
    get_all_resources, get_permission, get_permission_by_name, get_permission_by_resource_action,
    get_role_permissions, get_user_permissions, list_permissions, update_permission,
};
use crate::api::iam::middleware::auth_middleware::auth_required;

pub fn permission_routes<T: Clone + Send + Sync + 'static>(state: Arc<T>) -> Router {
    Router::new()
        // Public endpoints (read-only)
        .route("/", get(list_permissions::<T>))
        .route("/resources", get(get_all_resources::<T>))
        .route(
            "/resources/:resource/actions",
            get(get_actions_for_resource::<T>),
        )
        .route("/:id", get(get_permission::<T>))
        .route("/name/:name", get(get_permission_by_name::<T>))
        .route(
            "/resource/:resource/action/:action",
            get(get_permission_by_resource_action::<T>),
        )
        .route("/users/:user_id", get(get_user_permissions::<T>))
        .route("/roles/:role_id", get(get_role_permissions::<T>))
        // Protected endpoints
        .route("/", post(create_permission::<T>))
        .route("/:id", put(update_permission::<T>))
        .route("/:id", delete(delete_permission::<T>))
        .route("/users/:user_id/check", post(check_permission::<T>))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_required::<T>,
        ))
        .with_state(state)
}
