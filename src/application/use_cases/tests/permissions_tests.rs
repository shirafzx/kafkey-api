use mockall::predicate::*;
use std::sync::Arc;

use super::helpers::*;
use super::mocks::{MockAuditRepo, MockPermissionRepo};
use crate::application::use_cases::{audit::AuditUseCases, permissions::PermissionUseCases};
use crate::domain::entities::permission::NewPermissionEntity;

#[tokio::test]
async fn test_create_permission_success() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let permission_id = test_uuid();

    mock_perm_repo
        .expect_create()
        .withf(|perm: &NewPermissionEntity| {
            perm.name == "test_permission" && perm.resource == "users" && perm.action == "read"
        })
        .times(1)
        .returning(move |_| Ok(permission_id));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case
        .create_permission(
            actor_id,
            "test_permission".to_string(),
            "users".to_string(),
            "read".to_string(),
            Some("Test description".to_string()),
        )
        .await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), permission_id);
}

#[tokio::test]
async fn test_get_permission_by_id_success() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let permission_id = test_uuid();
    let expected_permission = create_test_permission(permission_id, "test", "users", "read");

    mock_perm_repo
        .expect_find_by_id()
        .with(eq(permission_id))
        .times(1)
        .returning(move |_| Ok(expected_permission.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case.get_permission_by_id(permission_id).await;

    // Assert
    assert!(result.is_ok());
    let permission = result.unwrap();
    assert_eq!(permission.id, permission_id);
    assert_eq!(permission.name, "test");
}

#[tokio::test]
async fn test_list_permissions_success() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let perm1 = create_test_permission(test_uuid(), "perm1", "users", "read");
    let perm2 = create_test_permission(test_uuid(), "perm2", "users", "write");
    let expected = vec![perm1.clone(), perm2.clone()];

    mock_perm_repo
        .expect_find_all()
        .times(1)
        .returning(move || Ok(expected.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case.list_permissions().await;

    // Assert
    assert!(result.is_ok());
    let permissions = result.unwrap();
    assert_eq!(permissions.len(), 2);
}

#[tokio::test]
async fn test_update_permission_success() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let permission_id = test_uuid();

    mock_perm_repo
        .expect_update()
        .with(
            eq(permission_id),
            eq(Some("updated_name".to_string())),
            eq(Some("Updated description".to_string())),
        )
        .times(1)
        .returning(|_, _, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case
        .update_permission(
            actor_id,
            permission_id,
            Some("updated_name".to_string()),
            Some("Updated description".to_string()),
        )
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_permission_success() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let permission_id = test_uuid();

    mock_perm_repo
        .expect_delete()
        .with(eq(permission_id))
        .times(1)
        .returning(|_| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case.delete_permission(actor_id, permission_id).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_permission_repository_error() {
    // Arrange
    let mut mock_perm_repo = MockPermissionRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();

    mock_perm_repo
        .expect_create()
        .times(1)
        .returning(|_| Err(anyhow::anyhow!("Database error")));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = PermissionUseCases::new(Arc::new(mock_perm_repo), audit_use_case);

    // Act
    let result = use_case
        .create_permission(
            actor_id,
            "test".to_string(),
            "users".to_string(),
            "read".to_string(),
            None,
        )
        .await;

    // Assert
    assert!(result.is_err());
}
