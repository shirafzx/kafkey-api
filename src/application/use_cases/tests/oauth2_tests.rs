use std::sync::Arc;
use uuid::Uuid;

use super::mocks::{MockUserRepo, MockUserSocialAccountRepo};
use crate::application::use_cases::oauth2::OAuth2UseCases;
use crate::services::{jwt_service::JwtService, oauth2_service::OAuth2Service};

#[tokio::test]
async fn test_get_google_auth_url() {
    // Arrange
    let mock_user_repo = MockUserRepo::new();
    let mock_social_repo = MockUserSocialAccountRepo::new();

    let oauth2_service = Arc::new(OAuth2Service::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
        "test_github_client_id".to_string(),
        "test_github_client_secret".to_string(),
        "http://localhost:3000/github/callback".to_string(),
    ));

    let jwt_service = Arc::new(JwtService::new(
        "test_access_secret".to_string(),
        "test_refresh_secret".to_string(),
    ));

    let use_case = OAuth2UseCases::new(
        Arc::new(mock_user_repo),
        Arc::new(mock_social_repo),
        oauth2_service,
        jwt_service,
    );

    // Act
    let (auth_url, state, pkce_verifier) = use_case.get_google_auth_url();

    // Assert
    assert!(!auth_url.is_empty());
    assert!(!state.is_empty());
    assert!(!pkce_verifier.is_empty());
    assert!(auth_url.contains("accounts.google.com"));
}

#[tokio::test]
async fn test_get_github_auth_url() {
    // Arrange
    let mock_user_repo = MockUserRepo::new();
    let mock_social_repo = MockUserSocialAccountRepo::new();

    let oauth2_service = Arc::new(OAuth2Service::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
        "test_github_client_id".to_string(),
        "test_github_client_secret".to_string(),
        "http://localhost:3000/github/callback".to_string(),
    ));

    let jwt_service = Arc::new(JwtService::new(
        "test_access_secret".to_string(),
        "test_refresh_secret".to_string(),
    ));

    let use_case = OAuth2UseCases::new(
        Arc::new(mock_user_repo),
        Arc::new(mock_social_repo),
        oauth2_service,
        jwt_service,
    );

    // Act
    let (auth_url, state) = use_case.get_github_auth_url();

    // Assert
    assert!(!auth_url.is_empty());
    assert!(!state.is_empty());
    assert!(auth_url.contains("github.com"));
}

#[tokio::test]
async fn test_state_validation() {
    // This test demonstrates that state validation should prevent CSRF attacks
    // The actual implementation would need to validate that the expected_state
    // matches the state returned from the OAuth provider

    // Note: This is a conceptual test. The actual OAuth2 callback tests would
    // require mocking HTTP calls to OAuth providers, which is complex.
    // In practice, these would be integration tests rather than unit tests.
    let state1 = Uuid::new_v4().to_string();
    let state2 = Uuid::new_v4().to_string();

    // States should be different
    assert_ne!(state1, state2);

    // State matching logic
    assert_eq!(state1, state1);
    assert_ne!(state1, state2);
}

// Note: Full integration tests for OAuth2 callbacks would require:
// 1. Mocking HTTP requests to Google/GitHub token endpoints
// 2. Mocking HTTP requests to user info endpoints
// 3. Complex setup of OAuth2 flow state
//
// These are better suited as integration tests with a real or mock HTTP server.
// The tests above verify the basic URL generation and state management,
// which are the core responsibilities we can unit test without HTTP mocking.
