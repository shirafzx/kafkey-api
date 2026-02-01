use std::sync::Arc;

use super::helpers::*;
use super::mocks::MockAuditRepo;
use crate::application::use_cases::audit::AuditUseCases;
use crate::domain::entities::audit_log::NewAuditLogEntity;

#[tokio::test]
async fn test_log_success() {
    // Arrange
    let mut mock_repo = MockAuditRepo::new();
    let actor_id = test_uuid();
    let target_id = test_uuid();

    mock_repo
        .expect_create()
        .withf(move |audit: &NewAuditLogEntity| {
            audit.actor_id == actor_id
                && audit.event_type == "TEST_EVENT"
                && audit.target_id == Some(target_id)
                && audit.resource == "test_resource"
                && audit.action == "test_action"
        })
        .times(1)
        .returning(|_| Ok(()));

    let use_case = AuditUseCases::new(Arc::new(mock_repo));

    // Act
    let result = use_case
        .log(
            actor_id,
            "TEST_EVENT",
            Some(target_id),
            "test_resource",
            "test_action",
            serde_json::json!({"key": "value"}),
        )
        .await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_log_repository_error() {
    // Arrange
    let mut mock_repo = MockAuditRepo::new();
    let actor_id = test_uuid();

    mock_repo
        .expect_create()
        .times(1)
        .returning(|_| Err(anyhow::anyhow!("Database error")));

    let use_case = AuditUseCases::new(Arc::new(mock_repo));

    // Act
    let result = use_case
        .log(
            actor_id,
            "TEST_EVENT",
            None,
            "test_resource",
            "test_action",
            serde_json::json!({}),
        )
        .await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Database error");
}

#[tokio::test]
async fn test_log_without_target_id() {
    // Arrange
    let mut mock_repo = MockAuditRepo::new();
    let actor_id = test_uuid();

    mock_repo
        .expect_create()
        .withf(move |audit: &NewAuditLogEntity| {
            audit.actor_id == actor_id && audit.target_id.is_none()
        })
        .times(1)
        .returning(|_| Ok(()));

    let use_case = AuditUseCases::new(Arc::new(mock_repo));

    // Act
    let result = use_case
        .log(
            actor_id,
            "TEST_EVENT",
            None,
            "test_resource",
            "test_action",
            serde_json::json!({}),
        )
        .await;

    // Assert
    assert!(result.is_ok());
}
