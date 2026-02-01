use mockall::predicate::*;
use std::sync::Arc;

use super::helpers::*;
use super::mocks::{MockBlacklistRepo, MockRoleRepo, MockUserRepo};
use crate::application::use_cases::auth::AuthUseCases;
use crate::domain::entities::user::NewUserEntity;
use crate::services::jwt_service::JwtService;

#[tokio::test]
async fn test_register_with_unique_credentials() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mut mock_role_repo = MockRoleRepo::new();
    let mock_blacklist_repo = MockBlacklistRepo::new();
    let user_id = test_uuid();
    let role_id = test_uuid();

    // Mock user creation
    mock_user_repo
        .expect_create()
        .withf(|user: &NewUserEntity| user.username == "newuser" && user.email == "new@example.com")
        .times(1)
        .returning(move |_| Ok(user_id));

    // Mock default role lookup
    let default_role = create_test_role(role_id, "user");
    mock_role_repo
        .expect_find_by_name()
        .with(eq("user".to_string()))
        .times(1)
        .returning(move |_| Ok(default_role.clone()));

    // Mock role assignment
    mock_user_repo
        .expect_assign_role()
        .with(eq(user_id), eq(role_id))
        .times(1)
        .returning(|_, _| Ok(()));

    let jwt_service = Arc::new(JwtService::new(
        "test_access_secret".to_string(),
        "test_refresh_secret".to_string(),
    ));

    let use_case = AuthUseCases::new(
        Arc::new(mock_user_repo),
        Arc::new(mock_role_repo),
        Arc::new(mock_blacklist_repo),
        jwt_service,
    );

    // Act
    let result = use_case
        .register(
            "newuser".to_string(),
            "new@example.com".to_string(),
            "New User".to_string(),
            "password123".to_string(),
            None,
        )
        .await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), user_id);
}

#[tokio::test]
async fn test_logout_with_valid_tokens() {
    // Arrange
    let mock_user_repo = MockUserRepo::new();
    let mock_role_repo = MockRoleRepo::new();
    let mut mock_blacklist_repo = MockBlacklistRepo::new();

    let jwt_service = Arc::new(JwtService::new(
        "test_access_secret".to_string(),
        "test_refresh_secret".to_string(),
    ));

    let user_id = test_uuid();
    let access_token = jwt_service
        .generate_access_token(user_id, vec![], vec![])
        .unwrap();
    let refresh_token = jwt_service.generate_refresh_token(user_id).unwrap();

    // Expect both tokens to be blacklisted
    mock_blacklist_repo
        .expect_add()
        .times(2)
        .returning(|_, _| Ok(()));

    let use_case = AuthUseCases::new(
        Arc::new(mock_user_repo),
        Arc::new(mock_role_repo),
        Arc::new(mock_blacklist_repo),
        jwt_service.clone(),
    );

    // Act
    let result = use_case.logout(access_token, Some(refresh_token)).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_generate_2fa_setup() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_role_repo = MockRoleRepo::new();
    let mock_blacklist_repo = MockBlacklistRepo::new();

    let user_id = test_uuid();
    let test_user = create_test_user(user_id, "testuser", "test@example.com");

    // Mock user lookup for 2FA setup
    mock_user_repo
        .expect_find_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(test_user.clone()));

    let jwt_service = Arc::new(JwtService::new(
        "test_access_secret".to_string(),
        "test_refresh_secret".to_string(),
    ));

    let use_case = AuthUseCases::new(
        Arc::new(mock_user_repo),
        Arc::new(mock_role_repo),
        Arc::new(mock_blacklist_repo),
        jwt_service,
    );

    // Act
    let result = use_case.generate_2fa_setup(user_id).await;

    // Assert
    assert!(result.is_ok());
    let (secret, provisioning_url) = result.unwrap();
    assert!(!secret.is_empty());
    assert!(!provisioning_url.is_empty());
    assert!(provisioning_url.contains("otpauth://totp"));
}
