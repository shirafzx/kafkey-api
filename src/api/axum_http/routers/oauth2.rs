use std::sync::Arc;

use axum::{
    Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::api::axum_http::response_utils::{error_response, success_response};
use crate::application::use_cases::oauth2::OAuth2UseCases;
use crate::domain::entities::user::UserEntity;
use crate::infrastructure::database::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::database::postgres::repositories::cached_user_repository::CachedUserRepository;
use crate::infrastructure::database::postgres::repositories::user_repository::UserPostgres;
use crate::infrastructure::database::postgres::repositories::user_social_account_repository::UserSocialAccountPostgres;
use crate::services::jwt_service::JwtService;
use crate::services::oauth2_service::OAuth2Service;

/// Response for OAuth2 login success
#[derive(Debug, Serialize)]
pub struct OAuth2LoginResponse {
    pub access_token: String,
    pub user: UserEntity,
}

/// Response for OAuth2 initiation
#[derive(Debug, Serialize)]
pub struct OAuth2InitResponse {
    pub auth_url: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkce_verifier: Option<String>,
}

pub fn routes(
    db_pool: Arc<PgPoolSquad>,
    jwt_service: Arc<JwtService>,
    oauth2_service: Arc<OAuth2Service>,
) -> Router {
    let pg_user_repo = Arc::new(UserPostgres::new(Arc::clone(&db_pool)));
    let user_repository = Arc::new(CachedUserRepository::new(pg_user_repo));
    let social_account_repository = Arc::new(UserSocialAccountPostgres::new(Arc::clone(&db_pool)));

    let oauth2_use_cases = Arc::new(OAuth2UseCases::new(
        user_repository,
        social_account_repository,
        oauth2_service,
        jwt_service,
    ));

    Router::new()
        .route("/api/v1/auth/oauth2/google/login", get(google_login))
        .route("/api/v1/auth/oauth2/google/callback", get(google_callback))
        .route("/api/v1/auth/oauth2/github/login", get(github_login))
        .route("/api/v1/auth/oauth2/github/callback", get(github_callback))
        .with_state(oauth2_use_cases)
}

/// Initiate Google OAuth2 flow
async fn google_login(State(oauth2_use_cases): State<Arc<OAuth2UseCases>>) -> impl IntoResponse {
    let (auth_url, state, pkce_verifier) = oauth2_use_cases.get_google_auth_url();

    success_response(
        "OAUTH2_INIT_SUCCESS",
        "Google OAuth2 flow initiated",
        OAuth2InitResponse {
            auth_url,
            state,
            pkce_verifier: Some(pkce_verifier),
        },
    )
}

/// Query parameters for Google OAuth2 callback
#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    #[serde(default)]
    pub code: Option<String>,
    pub state: String,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
    // In production, get these from session/redis
    #[serde(default)]
    pub expected_state: Option<String>,
    #[serde(default)]
    pub pkce_verifier: Option<String>,
}

/// Handle Google OAuth2 callback
async fn google_callback(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
    Query(params): Query<GoogleCallbackQuery>,
) -> impl IntoResponse {
    // 1. Check for errors from Google
    if let Some(error) = params.error {
        return error_response(
            StatusCode::BAD_REQUEST,
            "OAUTH2_GOOGLE_ERROR",
            &format!(
                "Google returned an error: {} - {}",
                error,
                params.error_description.unwrap_or_default()
            ),
            None,
        );
    }

    // 2. Validate code presence
    let code = match params.code {
        Some(c) => c,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_CODE",
                "missing field `code`. Google did not provide an authorization code in the callback.",
                None,
            );
        }
    };

    // 3. Validate expected_state presence
    let expected_state = match params.expected_state {
        Some(s) => s,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_EXPECTED_STATE",
                "missing field `expected_state`. In this test implementation, you must provide it in the query string.",
                None,
            );
        }
    };

    // 4. Validate pkce_verifier presence
    let pkce_verifier = match params.pkce_verifier {
        Some(v) => v,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_PKCE_VERIFIER",
                "missing field `pkce_verifier`. In this test implementation, you must provide it in the query string.",
                None,
            );
        }
    };

    match oauth2_use_cases
        .handle_google_callback(code, params.state, expected_state, pkce_verifier)
        .await
    {
        Ok((access_token, user)) => success_response(
            "OAUTH2_LOGIN_SUCCESS",
            "Google OAuth2 login successful",
            OAuth2LoginResponse { access_token, user },
        ),
        Err(e) => error_response(
            StatusCode::UNAUTHORIZED,
            "OAUTH2_LOGIN_FAILED",
            &e.to_string(),
            None,
        ),
    }
}

/// Initiate GitHub OAuth2 flow
async fn github_login(State(oauth2_use_cases): State<Arc<OAuth2UseCases>>) -> impl IntoResponse {
    let (auth_url, state) = oauth2_use_cases.get_github_auth_url();

    success_response(
        "OAUTH2_INIT_SUCCESS",
        "GitHub OAuth2 flow initiated",
        OAuth2InitResponse {
            auth_url,
            state,
            pkce_verifier: None,
        },
    )
}

/// Query parameters for GitHub OAuth2 callback
#[derive(Debug, Deserialize)]
pub struct GitHubCallbackQuery {
    #[serde(default)]
    pub code: Option<String>,
    pub state: String,
    // In production, get this from session/redis
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub error_description: Option<String>,
    #[serde(default)]
    pub expected_state: Option<String>,
}

/// Handle GitHub OAuth2 callback
async fn github_callback(
    State(oauth2_use_cases): State<Arc<OAuth2UseCases>>,
    Query(params): Query<GitHubCallbackQuery>,
) -> impl IntoResponse {
    // 1. Check for errors from GitHub
    if let Some(error) = params.error {
        return error_response(
            StatusCode::BAD_REQUEST,
            "OAUTH2_GITHUB_ERROR",
            &format!(
                "GitHub returned an error: {} - {}",
                error,
                params.error_description.unwrap_or_default()
            ),
            None,
        );
    }

    // 2. Validate code presence
    let code = match params.code {
        Some(c) => c,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_CODE",
                "missing field `code`. GitHub did not provide an authorization code in the callback.",
                None,
            );
        }
    };

    // 3. Validate expected_state presence
    let expected_state = match params.expected_state {
        Some(s) => s,
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "MISSING_EXPECTED_STATE",
                "missing field `expected_state`. In this test implementation, you must provide it in the query string.",
                None,
            );
        }
    };

    match oauth2_use_cases
        .handle_github_callback(code, params.state, expected_state)
        .await
    {
        Ok((access_token, user)) => success_response(
            "OAUTH2_LOGIN_SUCCESS",
            "GitHub OAuth2 login successful",
            OAuth2LoginResponse { access_token, user },
        ),
        Err(e) => error_response(
            StatusCode::UNAUTHORIZED,
            "OAUTH2_LOGIN_FAILED",
            &e.to_string(),
            None,
        ),
    }
}
