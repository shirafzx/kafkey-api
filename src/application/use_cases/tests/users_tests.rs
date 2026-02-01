use mockall::predicate::*;
use std::sync::Arc;

use super::helpers::*;
use super::mocks::{MockAuditRepo, MockUserRepo};
use crate::application::use_cases::{audit::AuditUseCases, users::UserUseCases};
use crate::domain::entities::user::{AdminUpdateUserParams, NewUserEntity};

#[tokio::test]
async fn test_create_user_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();

    mock_user_repo
        .expect_create()
        .withf(|user: &NewUserEntity| user.username == "testuser")
        .times(1)
        .returning(move |_| Ok(user_id));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let new_user = NewUserEntity {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        display_name: "Test User".to_string(),
        password_hash: "hash".to_string(),
        avatar_image_url: None,
        verification_token: None,
        verification_token_expires_at: None,
    };

    let result = use_case.create_user(new_user).await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), user_id);
}

#[tokio::test]
async fn test_get_user_by_id_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();
    let expected_user = create_test_user(user_id, "testuser", "test@example.com");

    mock_user_repo
        .expect_find_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(expected_user.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.get_user_by_id(user_id).await;

    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id, user_id);
    assert_eq!(user.username, "testuser");
}

#[tokio::test]
async fn test_assign_default_role_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let user_id = test_uuid();
    let role_id = test_uuid();

    mock_user_repo
        .expect_assign_role()
        .with(eq(user_id), eq(role_id))
        .times(1)
        .returning(|_, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case
        .assign_default_role(actor_id, user_id, role_id)
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_user_roles_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();
    let role1 = create_test_role(test_uuid(), "admin");
    let role2 = create_test_role(test_uuid(), "user");
    let expected = vec![role1.clone(), role2.clone()];

    mock_user_repo
        .expect_get_user_roles()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(expected.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.get_user_roles(user_id).await;

    // Assert
    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 2);
}

#[tokio::test]
async fn test_get_user_permissions_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();
    let perm1 = create_test_permission(test_uuid(), "perm1", "users", "read");
    let perm2 = create_test_permission(test_uuid(), "perm2", "users", "write");
    let expected = vec![perm1.clone(), perm2.clone()];

    mock_user_repo
        .expect_get_user_permissions()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(expected.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.get_user_permissions(user_id).await;

    // Assert
    assert!(result.is_ok());
    let permissions = result.unwrap();
    assert_eq!(permissions.len(), 2);
}

#[tokio::test]
async fn test_update_user_profile_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();

    mock_user_repo
        .expect_update_profile()
        .with(
            eq(user_id),
            eq(Some("New Display Name".to_string())),
            eq(Some("https://avatar.url".to_string())),
        )
        .times(1)
        .returning(|_, _, _| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case
        .update_user_profile(
            user_id,
            Some("New Display Name".to_string()),
            Some("https://avatar.url".to_string()),
        )
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_user_profile_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user_id = test_uuid();
    let user = create_verified_user(user_id, "testuser", "test@example.com");
    let roles = vec![create_test_role(test_uuid(), "admin")];

    let user_id_str = user_id.to_string();
    let user_clone = user.clone();
    let roles_clone = roles.clone();

    mock_user_repo
        .expect_find_by_id()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(user_clone.clone()));

    mock_user_repo
        .expect_get_user_roles()
        .with(eq(user_id))
        .times(1)
        .returning(move |_| Ok(roles_clone.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.get_current_user_profile(&user_id_str).await;

    // Assert
    assert!(result.is_ok());
    let profile = result.unwrap();
    assert_eq!(profile.username, "testuser");
    assert_eq!(profile.roles.len(), 1);
}

#[tokio::test]
async fn test_list_users_paginated_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let user1 = create_test_user(test_uuid(), "user1", "user1@example.com");
    let user2 = create_test_user(test_uuid(), "user2", "user2@example.com");
    let expected_users = vec![user1.clone(), user2.clone()];

    mock_user_repo
        .expect_find_paginated()
        .with(eq(10i64), eq(0i64))
        .times(1)
        .returning(move |_, _| Ok(expected_users.clone()));

    mock_user_repo
        .expect_count()
        .times(1)
        .returning(|| Ok(2i64));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.list_users_paginated(1, 10).await;

    // Assert
    assert!(result.is_ok());
    let (users, total) = result.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(total, 2);
}

#[tokio::test]
async fn test_delete_user_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let user_id = test_uuid();

    mock_user_repo
        .expect_delete()
        .with(eq(user_id))
        .times(1)
        .returning(|_| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.delete_user(actor_id, user_id).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_admin_update_user_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let user_id = test_uuid();
    let params = AdminUpdateUserParams {
        display_name: Some("Updated Name".to_string()),
        is_active: Some(true),
        ..Default::default()
    };

    mock_user_repo
        .expect_admin_update()
        .times(1)
        .returning(|_, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.admin_update_user(actor_id, user_id, params).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_remove_role_success() {
    // Arrange
    let mut mock_user_repo = MockUserRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let user_id = test_uuid();
    let role_id = test_uuid();

    mock_user_repo
        .expect_remove_role()
        .with(eq(user_id), eq(role_id))
        .times(1)
        .returning(|_, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = UserUseCases::new(Arc::new(mock_user_repo), audit_use_case);

    // Act
    let result = use_case.remove_role(actor_id, user_id, role_id).await;

    // Assert
    assert!(result.is_ok());
}
