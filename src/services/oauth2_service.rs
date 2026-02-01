use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUserInfo {
    pub id: i64,
    pub login: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

// Store config data to avoid typestate issues
struct OAuth2Config {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    auth_url: String,
    token_url: String,
}

pub struct OAuth2Service {
    google_config: OAuth2Config,
    github_config: OAuth2Config,
}

impl OAuth2Service {
    pub fn new(
        google_client_id: String,
        google_client_secret: String,
        google_redirect_url: String,
        github_client_id: String,
        github_client_secret: String,
        github_redirect_url: String,
    ) -> Self {
        Self {
            google_config: OAuth2Config {
                client_id: google_client_id,
                client_secret: google_client_secret,
                redirect_url: google_redirect_url,
                auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
            },
            github_config: OAuth2Config {
                client_id: github_client_id,
                client_secret: github_client_secret,
                redirect_url: github_redirect_url,
                auth_url: "https://github.com/login/oauth/authorize".to_string(),
                token_url: "https://github.com/login/oauth/access_token".to_string(),
            },
        }
    }

    pub fn get_google_auth_url(&self) -> (String, String, String) {
        let client = BasicClient::new(ClientId::new(self.google_config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.google_config.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(self.google_config.auth_url.clone()).unwrap())
            .set_token_uri(TokenUrl::new(self.google_config.token_url.clone()).unwrap())
            .set_redirect_uri(RedirectUrl::new(self.google_config.redirect_url.clone()).unwrap());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (
            auth_url.to_string(),
            csrf_token.secret().to_string(),
            pkce_verifier.secret().to_string(),
        )
    }

    pub fn get_github_auth_url(&self) -> (String, String) {
        let client = BasicClient::new(ClientId::new(self.github_config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.github_config.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(self.github_config.auth_url.clone()).unwrap())
            .set_token_uri(TokenUrl::new(self.github_config.token_url.clone()).unwrap())
            .set_redirect_uri(RedirectUrl::new(self.github_config.redirect_url.clone()).unwrap());

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .url();

        (auth_url.to_string(), csrf_token.secret().to_string())
    }

    pub async fn exchange_google_code(
        &self,
        code: String,
        pkce_verifier: String,
    ) -> anyhow::Result<(String, Option<String>, Option<i64>)> {
        let client = BasicClient::new(ClientId::new(self.google_config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.google_config.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(self.google_config.auth_url.clone()).unwrap())
            .set_token_uri(TokenUrl::new(self.google_config.token_url.clone()).unwrap())
            .set_redirect_uri(RedirectUrl::new(self.google_config.redirect_url.clone()).unwrap());

        let http_client = oauth2::reqwest::ClientBuilder::new()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&http_client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to exchange Google code: {}", e))?;

        Ok((
            token_result.access_token().secret().to_string(),
            token_result.refresh_token().map(|r| r.secret().to_string()),
            token_result.expires_in().map(|e| e.as_secs() as i64),
        ))
    }

    pub async fn exchange_github_code(
        &self,
        code: String,
    ) -> anyhow::Result<(String, Option<String>, Option<i64>)> {
        let client = BasicClient::new(ClientId::new(self.github_config.client_id.clone()))
            .set_client_secret(ClientSecret::new(self.github_config.client_secret.clone()))
            .set_auth_uri(AuthUrl::new(self.github_config.auth_url.clone()).unwrap())
            .set_token_uri(TokenUrl::new(self.github_config.token_url.clone()).unwrap())
            .set_redirect_uri(RedirectUrl::new(self.github_config.redirect_url.clone()).unwrap());

        let http_client = oauth2::reqwest::ClientBuilder::new()
            .redirect(oauth2::reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to exchange GitHub code: {}", e))?;

        Ok((
            token_result.access_token().secret().to_string(),
            token_result.refresh_token().map(|r| r.secret().to_string()),
            token_result.expires_in().map(|e| e.as_secs() as i64),
        ))
    }

    pub async fn get_google_user_info(&self, access_token: &str) -> anyhow::Result<GoogleUserInfo> {
        let http_client = reqwest::Client::new();
        let response = http_client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<GoogleUserInfo>()
            .await?;

        Ok(response)
    }

    pub async fn get_github_user_info(&self, access_token: &str) -> anyhow::Result<GitHubUserInfo> {
        let http_client = reqwest::Client::new();
        let response = http_client
            .get("https://api.github.com/user")
            .header("User-Agent", "Kafkey-API")
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<GitHubUserInfo>()
            .await?;

        Ok(response)
    }
}
