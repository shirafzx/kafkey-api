use crate::domain::entities::{permission::PermissionEntity, role::RoleEntity, user::UserEntity};
use chrono::Utc;
use uuid::Uuid;

/// Generate a test UUID
pub fn test_uuid() -> Uuid {
    Uuid::new_v4()
}

/// Create a test user entity
pub fn create_test_user(id: Uuid, username: &str, email: &str) -> UserEntity {
    UserEntity {
        id,
        username: username.to_string(),
        email: email.to_string(),
        display_name: "Test User".to_string(),
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$test".to_string(),
        avatar_image_url: None,
        is_active: Some(true),
        is_verified: Some(false),
        last_login_at: None,
        failed_login_attempts: 0,
        locked_at: None,
        verification_token: None,
        verification_token_expires_at: None,
        password_reset_token: None,
        password_reset_expires_at: None,
        two_factor_secret: None,
        two_factor_enabled: false,
        two_factor_backup_codes: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    }
}

/// Create a verified test user
pub fn create_verified_user(id: Uuid, username: &str, email: &str) -> UserEntity {
    let mut user = create_test_user(id, username, email);
    user.is_verified = Some(true);
    user
}

/// Create a test role entity
pub fn create_test_role(id: Uuid, name: &str) -> RoleEntity {
    RoleEntity {
        id,
        name: name.to_string(),
        description: Some(format!("{} role", name)),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    }
}

/// Create a test permission entity
pub fn create_test_permission(
    id: Uuid,
    name: &str,
    resource: &str,
    action: &str,
) -> PermissionEntity {
    PermissionEntity {
        id,
        name: name.to_string(),
        resource: resource.to_string(),
        action: action.to_string(),
        description: Some(format!("Permission to {} {}", action, resource)),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    }
}
