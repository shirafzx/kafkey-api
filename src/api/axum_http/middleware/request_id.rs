use axum::{
    body::Body,
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use tokio::task_local;
use uuid::Uuid;

task_local! {
    pub static REQUEST_ID: String;
}

pub async fn request_id_middleware(mut req: Request<Body>, next: Next) -> Response {
    let request_id = match req.headers().get("x-request-id") {
        Some(id) => id.to_str().unwrap_or_default().to_string(),
        None => Uuid::new_v4().to_string(),
    };

    let request_id_for_header = request_id.clone();

    // Set in request headers if not present
    if !req.headers().contains_key("x-request-id") {
        req.headers_mut().insert(
            "x-request-id",
            HeaderValue::from_str(&request_id).unwrap_or(HeaderValue::from_static("")),
        );
    }

    let mut response = REQUEST_ID.scope(request_id, next.run(req)).await;

    // Add to response headers
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id_for_header).unwrap_or(HeaderValue::from_static("")),
    );

    response
}
