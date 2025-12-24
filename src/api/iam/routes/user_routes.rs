use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::api::iam::handlers::user::{
    assign_role, check_permission, create_user, delete_user, get_user, get_user_by_email,
    get_user_by_username, get_user_permissions, get_user_roles, list_users, revoke_role,
    update_user,
};
use crate::api::iam::middleware::auth_middleware::auth_required;

pub fn user_routes<T: Clone + Send + Sync + 'static>(state: Arc<T>) -> Router {
    Router::new()
        // Public endpoints
        .route("/", post(create_user::<T>))
        .route("/", get(list_users::<T>))
        .route("/:id", get(get_user::<T>))
        .route("/username/:username", get(get_user_by_username::<T>))
        .route("/email/:email", get(get_user_by_email::<T>))
        // Protected endpoints
        .route("/:id", put(update_user::<T>))
        .route("/:id", delete(delete_user::<T>))
        .route("/:id/roles", post(assign_role::<T>))
        .route("/:id/roles", delete(revoke_role::<T>))
        .route("/:id/permissions/check", post(check_permission::<T>))
        .route("/:id/roles", get(get_user_roles::<T>))
        .route("/:id/permissions", get(get_user_permissions::<T>))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_required::<T>,
        ))
        .with_state(state)
}
