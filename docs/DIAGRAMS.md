# System Diagrams

This document contains visual representations of the Kafkey API system architecture, flows, and data models using Mermaid diagrams.

## Table of Contents

- [Architecture Diagrams](#architecture-diagrams)
  - [System Architecture](#system-architecture)
  - [Database Entity Relationship Diagram](#database-entity-relationship-diagram)
- [Authentication \u0026 Authorization](#authentication--authorization)
  - [Standard Login Flow](#standard-login-flow)
  - [OAuth2 Login Flow (Google)](#oauth2-login-flow-google)
  - [2FA Setup and Verification](#2fa-setup-and-verification)
  - [Request Authorization Flow](#request-authorization-flow)
- [Security Flows](#security-flows)
  - [Password Reset Flow](#password-reset-flow)
  - [Email Verification Flow](#email-verification-flow)
  - [JWT Token Refresh](#jwt-token-refresh)

---

## Architecture Diagrams

### System Architecture

High-level overview of the Kafkey API system components and their interactions.

```mermaid
graph TB
    subgraph "Client Layer"
        Client[Web/Mobile Client]
    end

    subgraph "API Layer - Axum Framework"
        Router[Axum Router]
        Middleware[Middleware Stack]
        Handlers[Route Handlers]
    end

    subgraph "Middleware Stack"
        RequestID[Request ID]
        RateLimit[Rate Limiter]
        CSRF[CSRF Protection]
        Auth[JWT Auth]
        Metrics[Prometheus Metrics]
        Tracing[Sentry Tracing]
    end

    subgraph "Application Layer"
        UseCases[Use Cases]
        Services[Services Layer]
    end

    subgraph "Services"
        JWTService[JWT Service]
        OAuth2Service[OAuth2 Service]
        PasswordService[Password Service]
        TOTPService[TOTP Service]
    end

    subgraph "Domain Layer"
        Entities[Domain Entities]
        Repos[Repository Interfaces]
    end

    subgraph "Infrastructure Layer"
        PostgresRepos[PostgreSQL Repositories]
        MongoRepos[MongoDB Repositories]
    end

    subgraph "Data Stores"
        Postgres[(PostgreSQL<br/>Primary Data)]
        MongoDB[(MongoDB<br/>Audit Logs)]
    end

    subgraph "External Services"
        GoogleOAuth[Google OAuth2]
        GitHubOAuth[GitHub OAuth2]
        Prometheus[Prometheus<br/>Monitoring]
        Sentry[Sentry<br/>Error Tracking]
    end

    Client --> Router
    Router --> Middleware
    Middleware --> RequestID --> RateLimit --> CSRF --> Auth --> Metrics --> Tracing
    Tracing --> Handlers
    Handlers --> UseCases
    UseCases --> Services
    UseCases --> Repos
    Services --> JWTService & OAuth2Service & PasswordService & TOTPService
    Repos --> PostgresRepos & MongoRepos
    PostgresRepos --> Postgres
    MongoRepos --> MongoDB
    OAuth2Service -.-> GoogleOAuth & GitHubOAuth
    Metrics -.-> Prometheus
    Tracing -.-> Sentry

    style Client fill:#e1f5ff
    style Postgres fill:#336791
    style MongoDB fill:#4DB33D
    style GoogleOAuth fill:#4285F4
    style GitHubOAuth fill:#181717
    style Prometheus fill:#E6522C
    style Sentry fill:#362D59
```

---

### Database Entity Relationship Diagram

PostgreSQL database schema showing relationships between tables.

```mermaid
erDiagram
    users ||--o{ user_roles : "has"
    users ||--o{ user_social_accounts : "has"
    roles ||--o{ user_roles : "assigned to"
    roles ||--o{ role_permissions : "has"
    permissions ||--o{ role_permissions : "granted to"

    users {
        uuid id PK
        varchar username UK
        varchar email UK
        varchar password_hash
        varchar display_name
        varchar avatar_image_url
        boolean is_active
        boolean is_verified
        timestamptz last_login_at
        int failed_login_attempts
        timestamptz locked_at
        varchar verification_token
        timestamptz verification_token_expires_at
        varchar password_reset_token
        timestamptz password_reset_expires_at
        varchar two_factor_secret
        boolean two_factor_enabled
        text[] two_factor_backup_codes
        timestamptz created_at
        timestamptz updated_at
    }

    roles {
        uuid id PK
        varchar name UK
        text description
        timestamptz created_at
        timestamptz updated_at
    }

    permissions {
        uuid id PK
        varchar name UK
        varchar resource
        varchar action
        text description
        timestamptz created_at
        timestamptz updated_at
    }

    user_roles {
        uuid user_id PK,FK
        uuid role_id PK,FK
        timestamptz assigned_at
    }

    role_permissions {
        uuid role_id PK,FK
        uuid permission_id PK,FK
        timestamptz created_at
    }

    user_social_accounts {
        uuid id PK
        uuid user_id FK
        varchar provider
        varchar provider_user_id UK
        varchar provider_email
        text access_token
        text refresh_token
        timestamp expires_at
        timestamp created_at
        timestamp updated_at
    }

    blacklisted_tokens {
        uuid jti PK
        timestamptz expires_at
        timestamptz created_at
    }
```

---

## Authentication & Authorization

### Standard Login Flow

Traditional username/password authentication with JWT token generation.

```mermaid
sequenceDiagram
    participant Client
    participant API as Axum API (Auth Router)
    participant Repos as UserRepository
    participant PasswordSvc as Password Service
    participant JWT as JWT Service
    participant Blacklist as Blacklist Repository

    Client->>API: POST /auth/login (emailOrUsername, password)
    API->>Repos: find_by_email() or find_by_username()
    Repos-->>API: User Data (password_hash, 2FA status)

    API->>PasswordSvc: verify(password, password_hash)
    PasswordSvc-->>API: Verification Result

    alt Invalid Credentials
        API->>Repos: Increment failed_login_attempts
        API-->>Client: 401 Unauthorized
    else Success (no 2FA)
        API->>Repos: get_user_roles(user_id)
        Repos-->>API: Roles List
        API->>Repos: get_user_permissions(user_id)
        Repos-->>API: Permissions List

        API->>JWT: generate_access_token(user_id, roles, perms)
        API->>JWT: generate_refresh_token(user_id)
        JWT-->>API: Signed Tokens (JTI included)

        API->>Repos: Update last_login_at, reset failed_attempts
        API-->>Client: 200 OK (access_token, refresh_token)
    else Success (2FA Enabled)
        API-->>Client: 200 OK (userId, requiresMfa: true)
        Note over Client,API: Client must call /auth/2fa/verify next
    end
```

---

### OAuth2 Login Flow (Google)

Social login using Google OAuth2 with PKCE and state validation.

```mermaid
sequenceDiagram
    participant Client
    participant API as Kafkey API
    participant OAuth2Svc as OAuth2 Service
    participant Google as Google OAuth2
    participant UserRepo as User Repository
    participant SocialRepo as Social Account Repository
    participant JWT as JWT Service

    Note over Client,API: Step 1: Initiate OAuth Flow
    Client->>API: GET /auth/oauth2/google/login
    API->>OAuth2Svc: get_google_auth_url()
    OAuth2Svc->>OAuth2Svc: Generate state token (CSRF)
    OAuth2Svc->>OAuth2Svc: Generate PKCE verifier & challenge
    OAuth2Svc-->>API: authUrl, state, pkceVerifier
    API-->>Client: 200 OK (authUrl, state, pkceVerifier)
    Note over Client: Store state and pkceVerifier

    Note over Client,Google: Step 2: User Authorization
    Client->>Google: Redirect to authUrl
    Google->>Google: User authenticates & authorizes
    Google-->>Client: Redirect to callback with code & state

    Note over Client,API: Step 3: Token Exchange & Account Linking
    Client->>API: GET /auth/oauth2/google/callback<br/>(code, state, expected_state, pkce_verifier)
    API->>OAuth2Svc: Validate state

    alt State Mismatch
        API-->>Client: 400 Bad Request (CSRF detected)
    else Valid State
        API->>OAuth2Svc: exchange_code_for_token(code, pkce_verifier)
        OAuth2Svc->>Google: POST /token (code, verifier)
        Google-->>OAuth2Svc: access_token, id_token
        OAuth2Svc->>Google: GET /userinfo (access_token)
        Google-->>OAuth2Svc: User profile (email, name, picture)
        OAuth2Svc-->>API: User profile data

        API->>SocialRepo: find_by_provider_and_id(google, provider_user_id)

        alt Social Account Exists
            SocialRepo-->>API: Social Account with user_id
            API->>SocialRepo: Update tokens
            API->>UserRepo: find_by_id(user_id)
        else Social Account Not Found
            API->>UserRepo: find_by_email(email)
            alt Email Exists & Verified
                UserRepo-->>API: Existing user
                API->>SocialRepo: Link social account to user
            else Email Not Verified
                API-->>Client: 403 Forbidden (Verify email first)
            else New User
                API->>UserRepo: create(username, email, display_name)
                UserRepo-->>API: New user_id
                API->>SocialRepo: create(user_id, provider, tokens)
            end
        end

        UserRepo-->>API: User entity
        API->>JWT: generate_tokens(user_id, roles, permissions)
        API-->>Client: 200 OK (access_token, user)
    end
```

---

### 2FA Setup and Verification

Two-factor authentication setup and login verification flow.

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant TOTPSvc as TOTP Service
    participant UserRepo as User Repository
    participant PasswordSvc as Password Service

    Note over Client,API: Setup Phase
    Client->>API: POST /auth/2fa/setup (Bearer token)
    API->>UserRepo: find_by_id(user_id)
    UserRepo-->>API: User (check 2FA not enabled)

    API->>TOTPSvc: generate_secret()
    TOTPSvc-->>API: base32_secret
    API->>TOTPSvc: get_qr_code_url(secret, email)
    TOTPSvc-->>API: otpauth://... URL
    API-->>Client: 200 OK (secret, qrCodeUrl)

    Note over Client: User scans QR in authenticator app

    Client->>API: POST /auth/2fa/confirm (secret, code)
    API->>TOTPSvc: verify_code(secret, code)

    alt Invalid Code
        API-->>Client: 400 Bad Request
    else Valid Code
        API->>PasswordSvc: generate_backup_codes(8)
        PasswordSvc-->>API: 8 hashed backup codes
        API->>UserRepo: Update user (enable 2FA, save secret & codes)
        API-->>Client: 200 OK (backupCodes array)
        Note over Client: User must save backup codes securely
    end

    Note over Client,API: Login Verification Phase
    Client->>API: POST /auth/login (credentials)
    API-->>Client: 200 OK (userId, requiresMfa: true)

    Client->>API: POST /auth/2fa/verify (userId, code)
    API->>UserRepo: find_by_id(userId)
    UserRepo-->>API: User (with 2FA secret)

    API->>TOTPSvc: verify_code(secret, code)

    alt Code Valid
        API-->>Client: 200 OK (tokens)
    else Code Invalid, Try Backup
        API->>PasswordSvc: verify_backup_code(code, stored_codes)
        alt Backup Valid
            API->>UserRepo: Remove used backup code
            API-->>Client: 200 OK (tokens)
        else All Invalid
            API-->>Client: 401 Unauthorized
        end
    end
```

---

### Request Authorization Flow

Token-based authorization with permission checking from JWT claims.

> [!TIP]
> This flow uses **Token-Based Authorization**. Permissions are embedded in the JWT during login, so subsequent requests **skip the database** for maximum performance.

```mermaid
sequenceDiagram
    participant Client
    participant MW as Middleware Stack
    participant AuthMW as Auth Middleware
    participant Handler as Route Handler
    participant Blacklist as Blacklist Repository
    participant DB as Repository/Database

    Client->>MW: Request with Bearer Token
    MW->>MW: Request ID Middleware: Generate x-request-id
    MW->>MW: Rate Limit: Check IP bucket (10 req/min)
    MW->>MW: CSRF Middleware: Validate CSRF token

    MW->>AuthMW: JWT Authentication
    AuthMW->>AuthMW: Extract token from Authorization header
    AuthMW->>AuthMW: Verify JWT signature (RS256)

    alt Token Invalid/Expired
        AuthMW-->>Client: 401 Unauthorized
    else Token Valid
        AuthMW->>Blacklist: is_blacklisted(jti)
        Blacklist-->>AuthMW: Check result

        alt Token Blacklisted
            AuthMW-->>Client: 401 Token Revoked
        else Token Active
            AuthMW->>AuthMW: Extract claims (user_id, roles, permissions)
            AuthMW->>Handler: Pass request with UserContext

            Handler->>Handler: Check required permission (from JWT claims)

            alt Permission Missing
                Handler-->>Client: 403 Forbidden
            else Permission Granted
                Handler->>DB: Query business data
                DB-->>Handler: Results
                Handler->>Handler: Wrap in ApiResponse
                Handler-->>Client: 200 OK (data, meta)
            end
        end
    end

    Note over MW,Client: All responses include x-request-id header
```

---

## Security Flows

### Password Reset Flow

Secure password reset using time-limited tokens sent via email.

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant UserRepo as User Repository
    participant Email as Email Service
    participant PasswordSvc as Password Service

    Note over Client,API: Request Reset
    Client->>API: POST /auth/forgot-password (email)
    API->>UserRepo: find_by_email(email)

    alt User Not Found
        API-->>Client: 200 OK (silent success for security)
        Note over API: Don't reveal if email exists
    else User Found
        UserRepo-->>API: User entity
        API->>PasswordSvc: generate_reset_token()
        PasswordSvc-->>API: hex_token (32 bytes)
        API->>UserRepo: Save token & expiry (1 hour)
        API->>Email: Send reset email with token link
        API-->>Client: 200 OK
    end

    Note over Client,Email: User clicks link in email

    Note over Client,API: Reset Password
    Client->>API: POST /auth/reset-password (token, newPassword)
    API->>UserRepo: find_by_reset_token(token)

    alt Token Invalid/Expired
        API-->>Client: 400 Invalid or expired token
    else Token Valid
        UserRepo-->>API: User entity
        API->>PasswordSvc: hash_password(newPassword)
        PasswordSvc-->>API: password_hash
        API->>UserRepo: Update password, clear reset token
        API-->>Client: 200 OK (Password reset successful)
    end
```

---

### Email Verification Flow

Email verification flow for new user registrations.

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant UserRepo as User Repository
    participant RoleRepo as Role Repository
    participant Email as Email Service

    Note over Client,API: Registration
    Client->>API: POST /auth/sign-up (username, email, password)
    API->>UserRepo: check username/email uniqueness
    API->>UserRepo: create(user data)
    UserRepo-->>API: user_id

    API->>RoleRepo: find_by_name("user")
    RoleRepo-->>API: default_role
    API->>UserRepo: assign_role(user_id, role_id)

    API->>API: Generate verification token (UUID)
    API->>UserRepo: Save token & expiry (24 hours)
    API->>Email: Send verification email with link
    API-->>Client: 201 Created (user_id)

    Note over Client,Email: User clicks verification link

    Note over Client,API: Verify Email
    Client->>API: GET /auth/verify-email?token=<token>
    API->>UserRepo: find_by_verification_token(token)

    alt Token Invalid/Expired
        API-->>Client: 400 Invalid or expired token
    else Token Valid
        UserRepo-->>API: User entity
        API->>UserRepo: mark_as_verified(), clear token
        API-->>Client: 200 OK (Email verified successfully)
    end
```

---

### JWT Token Refresh

Refreshing an expired access token using a valid refresh token.

```mermaid
sequenceDiagram
    participant Client
    participant API
    participant JWT as JWT Service
    participant Blacklist as Blacklist Repository
    participant UserRepo as User Repository

    Client->>API: POST /auth/refresh (refreshToken)
    API->>JWT: verify_token(refreshToken)

    alt Invalid Refresh Token
        API-->>Client: 401 Invalid refresh token
    else Valid Refresh Token
        JWT-->>API: Decoded claims (user_id, jti)
        API->>Blacklist: is_blacklisted(jti)

        alt Token Blacklisted
            API-->>Client: 401 Token revoked
        else Token Active
            Blacklist-->>API: Not blacklisted
            API->>UserRepo: find_by_id(user_id)
            UserRepo-->>API: User entity

            alt User Inactive/Locked
                API-->>Client: 403 Account disabled
            else User Active
                API->>UserRepo: get_user_roles(user_id)
                API->>UserRepo: get_user_permissions(user_id)
                UserRepo-->>API: Roles & Permissions

                API->>JWT: generate_access_token(user_id, roles, perms)
                JWT-->>API: New access token
                API-->>Client: 200 OK (accessToken, refreshToken)

                Note over Client: Old refresh token still valid<br/>until it expires or is blacklisted
            end
        end
    end
```

---

## Monitoring & Observability Flow

```mermaid
graph LR
    subgraph "Request Flow"
        Request[HTTP Request]
        RequestID[Request ID Middleware]
        Tracing[Sentry Middleware]
        Prometheus[Prometheus Metrics]
        Handler[Business Logic]
    end

    subgraph "Observability Stack"
        Logs[Structured Logs]
        Metrics[Metrics Store]
        APM[Sentry APM]
        Audit[Audit Logs MongoDB]
    end

    Request --> RequestID
    RequestID --> Tracing
    Tracing --> Prometheus
    Prometheus --> Handler

    Handler -.Log Context.-> Logs
    Handler -.Record Metric.-> Metrics
    Handler -.Trace Span.-> APM
    Handler -.Audit Event.-> Audit

    Logs -.Query Logs.-> Operator[DevOps/SRE]
    Metrics -.Dashboards.-> Operator
    APM -.Error Tracking.-> Operator
    Audit -.Compliance Reports.-> Operator

    style Request fill:#e1f5ff
    style Logs fill:#FFA500
    style Metrics fill:#E6522C
    style APM fill:#362D59
    style Audit fill:#4DB33D
```

---

**Last Updated**: February 2026
