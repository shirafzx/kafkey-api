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

| Column          | Type   | Relationships                                   |
| --------------- | ------ | ----------------------------------------------- |
| `role_id`       | `UUID` | **FK** -> `roles(id)` (ON DELETE CASCADE)       |
| `permission_id` | `UUID` | **FK** -> `permissions(id)` (ON DELETE CASCADE) |

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

## 📈 Indexes & Performance

Custom indexes created for query optimization:

1.  `idx_role_permissions_role_id` on `role_permissions(role_id)`
2.  `idx_role_permissions_permission_id` on `role_permissions(permission_id)`
3.  `idx_user_roles_user_id` on `user_roles(user_id)`
4.  `idx_user_roles_role_id` on `user_roles(role_id)`
5.  `idx_permissions_resource_action` on `permissions(resource, action)`
6.  `idx_blacklisted_tokens_expires_at` on `blacklisted_tokens(expires_at)` (Cleanup Job)
7.  `idx_users_verification_token` on `users(verification_token)` (Lookup)
8.  `idx_users_password_reset_token` on `users(password_reset_token)` (Lookup)
9.  `idx_users_created_at` on `users(created_at)` (Sorting)
