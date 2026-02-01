use anyhow::Result;

use crate::config::config_model::AuthSecret;

use super::{
    config_model::{Database, DotEnvyConfig, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
        sentry_dsn: std::env::var("SENTRY_DSN").ok(),
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
        mongodb_url: std::env::var("MONGODB_URL").expect("MONGODB_URL is invalid"),
        max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or("10".to_string())
            .parse()?,
        min_idle: std::env::var("DATABASE_MIN_IDLE")
            .ok()
            .map(|v| v.parse())
            .transpose()?,
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_auth_secret_env() -> Result<AuthSecret> {
    dotenvy::dotenv().ok();

    let authentication_secret = AuthSecret {
        secret: std::env::var("JWT_SECRET").expect("JWT_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET is invalid"),
    };

    Ok(authentication_secret)
}

pub fn get_oauth2_secrets_env() -> Result<crate::config::config_model::OAuth2Secrets> {
    dotenvy::dotenv().ok();

    let oauth2_secrets = crate::config::config_model::OAuth2Secrets {
        google_client_id: std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is missing"),
        google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET is missing"),
        google_redirect_url: std::env::var("GOOGLE_REDIRECT_URL").unwrap_or_else(|_| {
            "http://localhost:8080/api/v1/auth/oauth2/google/callback".to_string()
        }),
        github_client_id: std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID is missing"),
        github_client_secret: std::env::var("GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET is missing"),
        github_redirect_url: std::env::var("GITHUB_REDIRECT_URL").unwrap_or_else(|_| {
            "http://localhost:8080/api/v1/auth/oauth2/github/callback".to_string()
        }),
    };

    Ok(oauth2_secrets)
}
