use crate::api::axum_http::response_utils::error_response;
use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::CookieJar;

pub async fn csrf_middleware(jar: CookieJar, request: Request<Body>, next: Next) -> Response {
    let method = request.method();

    // Check if it's a state-changing request
    if matches!(
        method,
        &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH
    ) {
        let cookie_token = jar.get("csrf_token").map(|c| c.value());
        let header_token = request
            .headers()
            .get("X-CSRF-Token")
            .and_then(|h| h.to_str().ok());

        match (cookie_token, header_token) {
            (Some(c), Some(h)) if c == h => {
                // Tokens match, proceed
            }
            _ => {
                return error_response(
                    StatusCode::FORBIDDEN,
                    "CSRF_ERROR",
                    "Invalid or missing CSRF token",
                    None,
                )
                .into_response();
            }
        }
    }

    next.run(request).await
}
