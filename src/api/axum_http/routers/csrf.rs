use crate::api::axum_http::response_utils::success_response;
use axum::{Router, response::IntoResponse, routing::get};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use uuid::Uuid;

pub fn routes() -> Router {
    Router::new().route("/api/v1/csrf-token", get(get_csrf_token))
}

async fn get_csrf_token(jar: CookieJar) -> impl IntoResponse {
    let token = Uuid::new_v4().to_string();

    let cookie = Cookie::build(("csrf_token", token.clone()))
        .path("/")
        .http_only(false) // Must be readable by client for Double Submit
        .secure(false) // Set to true in production if HTTPS
        .same_site(axum_extra::extract::cookie::SameSite::Lax);

    (
        jar.add(cookie),
        success_response(
            "CSRF_TOKEN_GENERATED",
            "CSRF token generated",
            serde_json::json!({ "token": token }),
        ),
    )
}
