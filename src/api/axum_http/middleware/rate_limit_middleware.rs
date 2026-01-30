use crate::api::axum_http::response_utils::error_response;
use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Rate limit configuration
#[derive(Clone, Copy)]
pub struct RateLimitConfig {
    pub requests_per_window: u32,
    pub window_duration: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_window: 10,                  // 10 requests
            window_duration: Duration::from_secs(60), // per 60 seconds
        }
    }
}

pub struct RateLimitState {
    pub request_counts: HashMap<SocketAddr, (u32, Instant)>,
}

/// Simple in-memory rate limiting middleware
pub async fn rate_limit_middleware(
    config: RateLimitConfig,
    state: Arc<Mutex<RateLimitState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Extract IP address from connection info
    let addr = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0);

    if let Some(addr) = addr {
        let mut state = state.lock().unwrap();
        let now = Instant::now();

        let (count, start_time) = state.request_counts.entry(addr).or_insert((0, now));

        // Reset window if duration has passed
        if now.duration_since(*start_time) > config.window_duration {
            *count = 1;
            *start_time = now;
        } else {
            *count += 1;
        }

        if *count > config.requests_per_window {
            return error_response(
                StatusCode::TOO_MANY_REQUESTS,
                "RATE_LIMIT_EXCEEDED",
                "Too many requests, please try again later",
                None,
            );
        }
    }

    next.run(request).await
}

impl RateLimitState {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            request_counts: HashMap::new(),
        }))
    }
}
