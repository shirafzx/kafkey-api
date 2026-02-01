use mockall::predicate::*;
use std::sync::Arc;

use super::helpers::*;
use super::mocks::{MockAuditRepo, MockRoleRepo};
use crate::application::use_cases::{audit::AuditUseCases, roles::RoleUseCases};
use crate::domain::entities::role::NewRoleEntity;

#[tokio::test]
async fn test_create_role_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let role_id = test_uuid();

    mock_role_repo
        .expect_create()
        .withf(|role: &NewRoleEntity| {
            role.name == "admin" && role.description == Some("Admin role".to_string())
        })
        .times(1)
        .returning(move |_| Ok(role_id));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case
        .create_role(
            actor_id,
            "admin".to_string(),
            Some("Admin role".to_string()),
        )
        .await;

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), role_id);
}

#[tokio::test]
async fn test_get_role_by_id_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let role_id = test_uuid();
    let expected_role = create_test_role(role_id, "admin");

    mock_role_repo
        .expect_find_by_id()
        .with(eq(role_id))
        .times(1)
        .returning(move |_| Ok(expected_role.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case.get_role_by_id(role_id).await;

    // Assert
    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.id, role_id);
    assert_eq!(role.name, "admin");
}

#[tokio::test]
async fn test_list_roles_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let role1 = create_test_role(test_uuid(), "admin");
    let role2 = create_test_role(test_uuid(), "user");
    let expected = vec![role1.clone(), role2.clone()];

    mock_role_repo
        .expect_find_all()
        .times(1)
        .returning(move || Ok(expected.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case.list_roles().await;

    // Assert
    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 2);
}

#[tokio::test]
async fn test_update_role_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let role_id = test_uuid();

    mock_role_repo
        .expect_update()
        .with(
            eq(role_id),
            eq(Some("super_admin".to_string())),
            eq(Some("Super admin role".to_string())),
        )
        .times(1)
        .returning(|_, _, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case
        .update_role(
            actor_id,
            role_id,
            Some("super_admin".to_string()),
            Some("Super admin role".to_string()),
        )
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_role_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let role_id = test_uuid();

    mock_role_repo
        .expect_delete()
        .with(eq(role_id))
        .times(1)
        .returning(|_| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case.delete_role(actor_id, role_id).await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_assign_permission_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let role_id = test_uuid();
    let permission_id = test_uuid();

    mock_role_repo
        .expect_assign_permission()
        .with(eq(role_id), eq(permission_id))
        .times(1)
        .returning(|_, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case
        .assign_permission(actor_id, role_id, permission_id)
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_remove_permission_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mut mock_audit_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let role_id = test_uuid();
    let permission_id = test_uuid();

    mock_role_repo
        .expect_remove_permission()
        .with(eq(role_id), eq(permission_id))
        .times(1)
        .returning(|_, _| Ok(()));

    mock_audit_repo
        .expect_create()
        .times(1)
        .returning(|_| Ok(()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case
        .remove_permission(actor_id, role_id, permission_id)
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_role_permissions_success() {
    // Arrange
    let mut mock_role_repo = MockRoleRepo::new();
    let mock_audit_repo = MockAuditRepo::new();
    let role_id = test_uuid();
    let perm1 = create_test_permission(test_uuid(), "perm1", "users", "read");
    let perm2 = create_test_permission(test_uuid(), "perm2", "users", "write");
    let expected = vec![perm1.clone(), perm2.clone()];

    mock_role_repo
        .expect_get_permissions()
        .with(eq(role_id))
        .times(1)
        .returning(move |_| Ok(expected.clone()));

    let audit_use_case = Arc::new(AuditUseCases::new(Arc::new(mock_audit_repo)));
    let use_case = RoleUseCases::new(Arc::new(mock_role_repo), audit_use_case);

    // Act
    let result = use_case.get_role_permissions(role_id).await;

    // Assert
    assert!(result.is_ok());
    let permissions = result.unwrap();
    assert_eq!(permissions.len(), 2);
}
