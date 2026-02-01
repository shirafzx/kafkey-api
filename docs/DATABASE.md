# Database Schema Documentation

This document references the PostgreSQL schema used by the Kafkey API.

## Overview

- **Database**: PostgreSQL
- **ORM**: Diesel (Rust)
- **Primary Key**: UUID v4 (`gen_random_uuid()`)
- **Conventions**: `snake_case` for table and column names.

---

## 🏗️ Tables

### `users`

Stores user account information and authentication credentials.

| Column             | Type           | Constraints                          | Description                             |
| ------------------ | -------------- | ------------------------------------ | --------------------------------------- |
| `id`               | `UUID`         | **PK**, Default: `gen_random_uuid()` | Unique user identifier.                 |
| `username`         | `VARCHAR(30)`  | **UNIQUE**, `NOT NULL`               | Unique username for login.              |
| `email`            | `VARCHAR(100)` | **UNIQUE**, `NOT NULL`               | Unique email for communication & login. |
| `password_hash`    | `VARCHAR(255)` | `NOT NULL`                           | Argon2 hashed password.                 |
| `display_name`     | `VARCHAR(50)`  | `NOT NULL`                           | Public display name.                    |
| `avatar_image_url` | `VARCHAR(255)` | `NULL`                               | Optional URL to profile picture.        |
| `is_active`        | `BOOLEAN`      | Default: `true`                      | Soft delete / Account status flag.      |
| `is_verified`      | `BOOLEAN`      | Default: `false`                     | Email verification status.              |
| `created_at`       | `TIMESTAMPTZ`  | Default: `NOW()`                     | Account creation timestamp.             |
| `updated_at`       | `TIMESTAMPTZ`  | Default: `NOW()`                     | Last update timestamp.                  |

#### Security & Lifecycle Fields

| Column                          | Type           | Description                               |
| ------------------------------- | -------------- | ----------------------------------------- |
| `last_login_at`                 | `TIMESTAMPTZ`  | Timestamp of last successful login.       |
| `failed_login_attempts`         | `INT`          | Default: `0`. Counter for lockout policy. |
| `locked_at`                     | `TIMESTAMPTZ`  | Timestamp when account was locked.        |
| `verification_token`            | `VARCHAR(255)` | Token for email verification.             |
| `verification_token_expires_at` | `TIMESTAMPTZ`  | Expiry for verification token.            |
| `password_reset_token`          | `VARCHAR(255)` | Token for password recovery.              |
| `password_reset_expires_at`     | `TIMESTAMPTZ`  | Expiry for reset token.                   |
| `two_factor_secret`             | `VARCHAR(255)` | TOTP (2FA) secret key.                    |
| `two_factor_enabled`            | `BOOLEAN`      | Default: `false`. Master switch for 2FA.  |
| `two_factor_backup_codes`       | `TEXT[]`       | Array of hashed backup codes.             |

---

### `roles`

Defines high-level access roles (e.g., `admin`, `user`).

| Column        | Type          | Constraints                          | Description               |
| ------------- | ------------- | ------------------------------------ | ------------------------- |
| `id`          | `UUID`        | **PK**, Default: `gen_random_uuid()` | Unique role identifier.   |
| `name`        | `VARCHAR(50)` | **UNIQUE**, `NOT NULL`               | Human-readable role name. |
| `description` | `TEXT`        | `NULL`                               | Optional description.     |
| `created_at`  | `TIMESTAMPTZ` | Default: `NOW()`                     | Role creation timestamp.  |
| `updated_at`  | `TIMESTAMPTZ` | Default: `NOW()`                     | Last update timestamp.    |

---

### `permissions`

Defines granular capabilities (e.g., `users.read`).

| Column        | Type           | Constraints                          | Description                          |
| ------------- | -------------- | ------------------------------------ | ------------------------------------ |
| `id`          | `UUID`         | **PK**, Default: `gen_random_uuid()` | Unique permission identifier.        |
| `name`        | `VARCHAR(100)` | **UNIQUE**, `NOT NULL`               | Slug name (e.g., `users.create`).    |
| `resource`    | `VARCHAR(100)` | `NOT NULL`                           | Target resource grouping.            |
| `action`      | `VARCHAR(50)`  | `NOT NULL`                           | Operation type (create, read, etc.). |
| `description` | `TEXT`         | `NULL`                               | Optional description.                |
| `created_at`  | `TIMESTAMPTZ`  | Default: `NOW()`                     | Permission creation timestamp.       |
| `updated_at`  | `TIMESTAMPTZ`  | Default: `NOW()`                     | Last update timestamp.               |

---

### `user_roles` (Join Table)

Many-to-Many relationship between Users and Roles.

| Column        | Type          | Relationships                             |
| ------------- | ------------- | ----------------------------------------- |
| `user_id`     | `UUID`        | **FK** -> `users(id)` (ON DELETE CASCADE) |
| `role_id`     | `UUID`        | **FK** -> `roles(id)` (ON DELETE CASCADE) |
| `assigned_at` | `TIMESTAMPTZ` | When the role was assigned.               |

**Primary Key**: (`user_id`, `role_id`)

---

### `role_permissions` (Join Table)

Many-to-Many relationship between Roles and Permissions.

| Column          | Type          | Relationships                                  |
| --------------- | ------------- | ---------------------------------------------- |
| `role_id`       | `UUID`        | **FK** → `roles(id)` (ON DELETE CASCADE)       |
| `permission_id` | `UUID`        | **FK** → `permissions(id)` (ON DELETE CASCADE) |
| `created_at`    | `TIMESTAMPTZ` | When the permission was assigned to the role.  |

**Primary Key**: (`role_id`, `permission_id`)

---

### `blacklisted_tokens`

Stores revoked JWT JTI (IDs) for secure logout.

| Column       | Type          | Constraints      | Description                             |
| ------------ | ------------- | ---------------- | --------------------------------------- |
| `jti`        | `UUID`        | **PK**           | Unique JWT ID from the token structure. |
| `expires_at` | `TIMESTAMPTZ` | `NOT NULL`       | When the token would naturally expire.  |
| `created_at` | `TIMESTAMPTZ` | Default: `NOW()` | When it was blacklisted.                |

---

### `user_social_accounts`

Stores OAuth2 social login credentials for Google and GitHub authentication.

| Column             | Type           | Constraints                              | Description                                  |
| ------------------ | -------------- | ---------------------------------------- | -------------------------------------------- |
| `id`               | `UUID`         | **PK**, Default: `gen_random_uuid()`     | Unique social account identifier.            |
| `user_id`          | `UUID`         | **FK** → `users(id)` (ON DELETE CASCADE) | Associated user account.                     |
| `provider`         | `VARCHAR(50)`  | `NOT NULL`                               | OAuth provider (e.g., `google`, `github`).   |
| `provider_user_id` | `VARCHAR(255)` | `NOT NULL`                               | Unique user ID from the OAuth provider.      |
| `provider_email`   | `VARCHAR(255)` | `NULL`                                   | Email address from OAuth provider.           |
| `access_token`     | `TEXT`         | `NULL`                                   | OAuth access token (encrypted recommended).  |
| `refresh_token`    | `TEXT`         | `NULL`                                   | OAuth refresh token (encrypted recommended). |
| `expires_at`       | `TIMESTAMP`    | `NULL`                                   | When the access token expires.               |
| `created_at`       | `TIMESTAMP`    | Default: `CURRENT_TIMESTAMP`             | When the account was linked.                 |
| `updated_at`       | `TIMESTAMP`    | Default: `CURRENT_TIMESTAMP`             | Last token refresh/update.                   |

**Unique Constraint**: (`provider`, `provider_user_id`)

---

## 📈 Indexes & Performance

Custom indexes created for query optimization:

### PostgreSQL Indexes

1.  `idx_role_permissions_role_id` on `role_permissions(role_id)` — Fast role permission lookups
2.  `idx_role_permissions_permission_id` on `role_permissions(permission_id)` — Permission usage tracking
3.  `idx_user_roles_user_id` on `user_roles(user_id)` — User role queries
4.  `idx_user_roles_role_id` on `user_roles(role_id)` — Role membership queries
5.  `idx_permissions_resource_action` on `permissions(resource, action)` — Permission lookups
6.  `idx_blacklisted_tokens_expires_at` on `blacklisted_tokens(expires_at)` — Cleanup job efficiency
7.  `idx_users_verification_token` on `users(verification_token)` — Email verification
8.  `idx_users_password_reset_token` on `users(password_reset_token)` — Password reset
9.  `idx_users_created_at` on `users(created_at)` — User listing and sorting
10. `idx_user_social_accounts_user_id` on `user_social_accounts(user_id)` — User OAuth accounts
11. `idx_user_social_accounts_provider` on `user_social_accounts(provider, provider_user_id)` — OAuth login

---

## 🗄️ MongoDB (Audit Logs)

The audit trail is stored separately in MongoDB for scalability and to avoid performance impact on the primary PostgreSQL database.

### Collection: `audit_logs`

Stores comprehensive audit trail of all system actions.

| Field        | Type       | Description                                      |
| ------------ | ---------- | ------------------------------------------------ |
| `_id`        | `ObjectId` | MongoDB auto-generated unique identifier.        |
| `user_id`    | `String`   | UUID of the user who performed the action.       |
| `action`     | `String`   | Action performed (e.g., `user.create`).          |
| `resource`   | `String`   | Resource affected (e.g., `user`, `role`).        |
| `target_id`  | `String`   | Optional UUID of the specific resource affected. |
| `timestamp`  | `DateTime` | When the action occurred (UTC).                  |
| `ip_address` | `String`   | Optional IP address of the requester.            |
| `user_agent` | `String`   | Optional browser/client user agent.              |
| `metadata`   | `Object`   | Optional JSON object with additional context.    |

### Indexes

- `user_id` — Fast user activity queries
- `action` — Filter by action type
- `timestamp` — Time-based queries and retention policies
- `target_id` — Resource-specific audit trails

---

## 🔄 Database Migrations

Database schema is managed using **Diesel CLI** migrations. All migrations are located in:

```
src/infrastructure/database/postgres/migrations/
```

### Running Migrations

```bash
# Run all pending migrations
diesel migration run

# Rollback the last migration
diesel migration revert

# Generate a new migration
diesel migration generate migration_name
```

### Migration History

1. `2025-12-03-171646_init_database` — Initial database setup
2. `2026-01-30-145603_create_iam_tables` — IAM tables (roles, permissions)
3. `2026-01-30-145646_update_users_table` — Enhanced user fields
4. `2026-01-30-165146_create_blacklisted_tokens` — JWT blacklist
5. `2026-01-30-171845_add_missing_permissions` — Seed permissions
6. `2026-01-30-174528_add_account_lockout_fields` — Account security
7. `2026-01-31-022029_add_verification_fields` — Email verification
8. `2026-01-31-023912_add_password_reset_fields` — Password reset
9. `2026-01-31-025311_add_2fa_fields` — Two-factor authentication
10. `2026-01-31-062153_add_performance_indexes` — Query optimization
11. `2026-01-31-165223_create_user_social_accounts` — OAuth2 integration

---

## 🔐 Security Considerations

### Password Storage

- All passwords are hashed using **Argon2id** before storage
- Never store plaintext passwords
- `password_hash` column stores the Argon2 hash with embedded salt

### Token Management

- JWT tokens use **RS256** signing algorithm
- Tokens include JTI (JWT ID) for blacklisting capability
- Refresh tokens have longer expiration than access tokens
- Expired tokens are automatically cleaned from blacklist (hourly job)

### OAuth2 Tokens

- Social account tokens should be encrypted at rest (recommended)
- Tokens are refreshed automatically when expired
- Failed OAuth flows don't create partial accounts

### Account Security

- Account lockout after 5 failed login attempts
- Verification tokens expire after 24 hours
- Password reset tokens expire after 1 hour
- 2FA backup codes are hashed before storage

---

## 📊 Database Maintenance

### Automated Cleanup

Background jobs run automatically:

- **Token Blacklist Cleanup**: Every 1 hour — Removes expired JTI entries

### Manual Maintenance

```sql
-- Clean up expired verification tokens
UPDATE users
SET verification_token = NULL, verification_token_expires_at = NULL
WHERE verification_token_expires_at < NOW();

-- Clean up expired password reset tokens
UPDATE users
SET password_reset_token = NULL, password_reset_expires_at = NULL
WHERE password_reset_expires_at < NOW();

-- Archive old audit logs (MongoDB)
db.audit_logs.deleteMany({
  timestamp: { $lt: new Date(Date.now() - 90*24*60*60*1000) } // >90 days
});
```

### Backup Recommendations

- **PostgreSQL**: Daily automated backups with point-in-time recovery
- **MongoDB**: Daily snapshots of audit collection
- **Retention**: Keep backups for at least 30 days
- **Testing**: Regularly test backup restoration procedures

---

**Last Updated**: February 2026
