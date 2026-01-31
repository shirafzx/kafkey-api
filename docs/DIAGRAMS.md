# System Sequence Diagrams

## Authentication Flow (Login)

```mermaid
sequenceDiagram
    participant Client
    participant API as Axum API (Auth Router)
    participant Repos as CachedUserRepository
    participant JWT as JWT Service

    Client->>API: POST /auth/login (emailOrUsername, password)
    API->>Repos: Fetch user by email or username
    Repos-->>API: User Data (hashed password, salt)
    API->>API: Verify password with Argon2

    alt Invalid Credentials
        API-->>Client: 401 Unauthorized (ApiErrorResponse)
    else Success
        API->>Repos: get_user_roles(user_id)
        Repos-->>API: Roles List
        API->>Repos: get_user_permissions(user_id) (Cached)
        Repos-->>API: Permissions List

        API->>JWT: Generate Access Token (Roles + Perms)
        API->>JWT: Generate Refresh Token (7d)
        JWT-->>API: Signed Tokens
        API-->>Client: 200 OK (ApiResponse with tokens)
    end
```

## Authorization & Request Processing

> [!TIP]
> This flow uses **Token-Based Authorization**. Permissions are embedded in the JWT during login (using the Cache), so subsequent requests **skip both the DB and the Cache** for maximum performance.

```mermaid
sequenceDiagram
    participant Client
    participant MW as Middleware (Tracing, Rate Limit, Auth)
    participant Guard as PBAC Guard
    participant Handler as Route Handler
    participant DB as Repository/Database

    Client->>MW: Request with Bearer Token
    MW->>MW: Tracing: Generate x-request-id
    MW->>MW: Rate Limit: Check IP bucket
    MW->>MW: Auth: Validate JWT & Extract Claims (Permissions included)

    alt Auth Failed
        MW-->>Client: 401 Unauthorized
    else Success
        MW->>Guard: Check Permission (from JWT Claims)
        alt Forbidden
            Guard-->>Client: 403 Forbidden
        else Authorized
            Guard->>Handler: Invoke Handler
            Handler->>DB: Query Business Data
            DB-->>Handler: Results
            Handler->>Handler: Wrap in ApiResponse
            Handler-->>Client: 200 OK
        end
    end
```
