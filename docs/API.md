# Kafkey API Documentation

## Base URL

All API endpoints are prefixed with `/api/v1`.

```
Base URL: http://localhost:8080/api/v1
```

## Global Conventions

### Case Convention

All JSON request and response keys use **camelCase**.

### Request Headers

| Header          | Description                                                                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization` | `Bearer <access_token>` (required for protected routes)                                                                                           |
| `X-CSRF-Token`  | CSRF token matching the `csrf_token` cookie (required for state-changing requests)                                                                |
| `x-request-id`  | **(Optional)** Unique ID for tracking requests. If provided by the client, it will be propagated. If missing, the server generates a new UUID v4. |

> [!TIP]
> You can find the `x-request-id` in the **response headers** or in the `meta` object of any JSON response.

## Response Format

All responses follow a standardized envelope structure.

### Success Response (200/201 OK)

```json
{
  "success": true,
  "code": "SUCCESS_CODE",
  "message": "Human readable message",
  "data": { ... },
  "meta": {
    "requestId": "uuid-v4-string",
    "timestamp": "2026-01-31T01:23:45Z",
    "version": "1.0"
  }
}
```

### Error Response (4xx/5xx)

```json
{
  "success": false,
  "code": "VALIDATION_ERROR",
  "message": "Input validation failed",
  "errors": [
    {
      "field": "email",
      "reason": "must be a valid email"
    },
    {
      "field": "password",
      "reason": "must be at least 8 characters"
    }
  ],
  "meta": {
    "requestId": "uuid-v4-string",
    "timestamp": "2026-01-31T01:23:45Z",
    "version": "1.0"
  }
}
```

### Pagination Structure

For list endpoints, the `data` field contains:

```json
{
  "items": [...],
  "page": 1,
  "pageSize": 20,
  "totalItems": 120,
  "totalPages": 6,
  "hasNext": true,
  "hasPrev": false
}
```

---

## Security

### CSRF Protection

Kafkey API implements CSRF protection using the **Double Submit Cookie** pattern. This is required for all state-changing requests (`POST`, `PUT`, `DELETE`, `PATCH`).

#### How to use:

1.  **Fetch a CSRF Token**: Call `GET /api/v1/auth/csrf-token`. This will:
    - Return a JSON response containing the `token`.
    - Set a `csrf_token` cookie in your browser/client.
2.  **Include the Token in Requests**: For all subsequent state-changing requests:
    - Extract the token value (either from the JSON response or the `csrf_token` cookie).
    - Add it as a header: `X-CSRF-Token: <token_value>`.
3.  **Validation**: The server will compare the value in the `X-CSRF-Token` header with the value in the `csrf_token` cookie. If they do not match or are missing, a `403 Forbidden` error will be returned.

> [!NOTE]
> The `csrf_token` cookie is NOT `HttpOnly`, allowing client-side scripts to read it and include it in the header.

### Rate Limiting

A global rate limit is applied to all endpoints to prevent abuse:

- **Limit**: 10 requests per minute per IP address.
- **Header**: Exceeding the limit returns `429 Too Many Requests` with a `Retry-After` header.

---

## Endpoints

### Authentication

#### GET /auth/csrf-token

Returns a new CSRF token and sets the `csrf_token` cookie.

**Response Data:**

```json
{
  "token": "uuid-v4-token"
}
```

---

#### POST /auth/sign-up

Register a new user account.

**Request Body:**

```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "displayName": "John Doe",
  "password": "SecurePass123!",
  "avatarImageUrl": "https://example.com/avatar.png"
}
```

#### POST /auth/login

Authenticate and receive JWT tokens.

**Request Body:**

```json
{
  "emailOrUsername": "john@example.com",
  "password": "SecurePass123!"
}
```

**Response Data:**

```json
{
  "userId": "uuid",
  "accessToken": "jwt-token",
  "refreshToken": "jwt-token"
}
```

#### POST /auth/refresh

Refresh an expired access token.

**Request Body:**

```json
{
  "refreshToken": "jwt-token"
}
```

---

#### POST /auth/logout (Protected)

Revoke the current access token and an optional refresh token by adding them to the blacklist.

**Request Body:**

```json
{
  "refreshToken": "jwt-token"
}
```

---

#### GET /auth/verify-email

Verify a user's email address using a token.

**Query Params:**

- `token`: The verification token sent via email.

---

#### POST /auth/resend-verification

Resend the verification email to a user.

**Request Body:**

```json
{
  "emailOrUsername": "john@example.com"
}
```

---

#### POST /auth/2fa/setup (Protected)

Begin 2FA setup. Returns a TOTP secret and a provisioning URL for QR codes.

**Response Data:**

```json
{
  "secret": "JBSWY3DPEHPK3PXP",
  "qrCodeUrl": "otpauth://totp/KafkeyAPI:john@example.com?secret=JBSWY3DPEHPK3PXP&issuer=KafkeyAPI"
}
```

---

#### POST /auth/2fa/confirm (Protected)

Confirm 2FA setup by providing the first 6-digit code. Returns backup codes.

**Request Body:**

```json
{
  "secret": "JBSWY3DPEHPK3PXP",
  "code": "123456"
}
```

**Response Data:**

```json
{
  "backupCodes": ["ABCDEF12", "GHIJKL34", ...]
}
```

---

#### POST /auth/2fa/verify

Verify 2FA code during login if required.

**Request Body:**

```json
{
  "userId": "uuid",
  "code": "123456"
}
```

**Response Data (AuthResponse):**

```json
{
  "userId": "uuid",
  "accessToken": "jwt-token",
  "refreshToken": "jwt-token"
}
```

---

#### POST /auth/2fa/disable (Protected)

Disable 2FA for the authenticated user.

**Request Body:**

```json
{
  "code": "123456"
}
```

---

#### POST /auth/forgot-password

Request a password reset. A reset token will be generated and would be sent to the user's email.

**Request Body:**

```json
{
  "email": "john@example.com"
}
```

---

#### POST /auth/reset-password

Reset a user's password using a valid reset token.

**Request Body:**

```json
{
  "token": "reset-token-hex",
  "newPassword": "NewSecurePass123!"
}
```

---

### User Management

#### GET /users/me (Protected)

Get current authenticated user's profile.

**Response Data:**

```json
{
  "id": "uuid",
  "username": "johndoe",
  "email": "john@example.com",
  "displayName": "John Doe",
  "avatarImageUrl": "https://example.com/avatar.png",
  "isActive": true,
  "isVerified": true
}
```

---

#### PUT /users/me (Protected)

Update current authenticated user's profile.

**Request Body:**

```json
{
  "displayName": "New Name",
  "avatarImageUrl": "https://newurl.com/img.png"
}
```

---

#### GET /users (Admin Only)

List all users with pagination. Requires `users.read` permission.

**Query Params:**

- `page`: Page number (default: 1)
- `pageSize`: Items per page (default: 20, max: 100)

**Response Data:** (See [Pagination Structure](#pagination-structure))

---

#### GET /users/{id} (Admin Only)

Get details of a specific user. Requires `users.read` permission.

---

#### PUT /users/{id} (Admin Only)

Update user details. Requires `users.update` permission.

**Request Body:**

```json
{
  "displayName": "Updated Name",
  "avatarImageUrl": "https://example.com/new.png",
  "isActive": true,
  "isVerified": true
}
```

---

#### DELETE /users/{id} (Admin Only)

Delete a user. Requires `users.delete` permission.

---

#### GET /users/{id}/roles (Admin Only)

List roles assigned to a specific user. Requires `users.read` permission.

---

#### POST /users/{id}/roles (Admin Only)

Assign a role to a user. Requires `users.update` permission.

**Request Body:**

```json
{
  "roleId": "uuid-of-role"
}
```

---

#### DELETE /users/{id}/roles/{roleId} (Admin Only)

Remove a role from a user. Requires `users.update` permission.

---

#### GET /users/{id}/permissions (Admin Only)

List all effective permissions for a user (aggregated from all roles). Requires `users.read` permission.

---

### Role Management (Admin Only)

#### GET /roles

List all defined roles. Requires `roles.read` permission.

---

#### POST /roles

Create a new role. Requires `roles.create` permission.

**Request Body:**

```json
{
  "name": "moderator",
  "description": "Can moderate user content"
}
```

---

#### GET /roles/{id}

Get details of a specific role. Requires `roles.read` permission.

---

#### PUT /roles/{id}

Update a role's name or description. Requires `roles.update` permission.

**Request Body:**

```json
{
  "name": "newname",
  "description": "new description"
}
```

---

#### DELETE /roles/{id}

Delete a role. Requires `roles.delete` permission.

---

#### GET /roles/{id}/permissions

List permissions assigned to a specific role. Requires `roles.read` permission.

---

#### POST /roles/{id}/permissions

Assign a permission to a role. Requires `roles.update` permission.

**Request Body:**

```json
{
  "permissionId": "uuid-of-permission"
}
```

---

#### DELETE /roles/{id}/permissions/{permissionId}

Remove a permission from a role. Requires `roles.update` permission.

---

### Permission Management (Admin Only)

#### GET /permissions

List all defined permissions. Requires `permissions.read` permission.

---

#### POST /permissions

Create a new permission. Requires `permissions.create` permission.

**Request Body:**

```json
{
  "name": "User Content Write",
  "resource": "content",
  "action": "write",
  "description": "Ability to create or edit content"
}
```

---

#### GET /permissions/{id}

Get details of a specific permission. Requires `permissions.read` permission.

---

#### PUT /permissions/{id}

Update a permission's name or description. Requires `permissions.update` permission.

**Request Body:**

```json
{
  "name": "New Permission Name",
  "description": "Updated description"
}
```

---

#### DELETE /permissions/{id}

Delete a permission. Requires `permissions.delete` permission.

---

### Monitoring & Health

#### GET /health-check

Returns system health status. This endpoint is excluded from rate limiting and metrics collection.

**Response:**

```json
{
  "success": true,
  "code": "HEALTH_OK",
  "message": "Service is healthy",
  "data": {
    "status": "healthy",
    "timestamp": "2026-02-01T14:19:06Z"
  },
  "meta": {
    "requestId": "uuid-v4",
    "timestamp": "2026-02-01T14:19:06Z",
    "version": "1.0"
  }
}
```

---

#### GET /metrics

Returns Prometheus-formatted metrics for monitoring. This endpoint is excluded from rate limiting and metrics collection to avoid recursive tracking.

**Response Format:** Prometheus text exposition format

**Metrics Include:**

- HTTP request counts and durations
- Response status code distributions
- Active request counts
- Custom application metrics

**Example Output:**

```
# HELP http_requests_total Total number of HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",endpoint="/api/v1/users"} 1234

# HELP http_request_duration_seconds HTTP request latency
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{method="POST",endpoint="/api/v1/auth/login",le="0.005"} 100
```

> [!NOTE]
> This endpoint is typically used by Prometheus to scrape metrics. Configure your Prometheus server to poll this endpoint at your desired interval (e.g., every 15 seconds).

---

### OAuth2 Authentication

Kafkey API supports OAuth2 authentication with Google and GitHub, allowing users to sign in using their existing accounts.

#### Security Features

- **Email Verification Required**: Only auto-links OAuth accounts to existing users if the user's email is already verified.
- **CSRF Protection**: State token validation prevents cross-site request forgery attacks.
- **PKCE Support**: Google OAuth uses Proof Key for Code Exchange (PKCE) for enhanced security.

#### Account Linking Logic

When a user authenticates via OAuth2:

1. **Existing OAuth Account**: If the social account is already linked, update tokens and log in.
2. **Email Match + Verified**: If email matches an existing verified user, link the social account.
3. **Email Not Verified**: Reject auto-linking for security (user must verify email manually first or login with credentials).
4. **New User**: Create a new account and link the social login.

---

#### GET /auth/oauth2/google/login

Initiate Google OAuth2 authentication flow.

**Response Data:**

```json
{
  "authUrl": "https://accounts.google.com/o/oauth2/v2/auth?...",
  "state": "csrf-state-token",
  "pkceVerifier": "pkce-verifier-string"
}
```

**Usage:**

1. Call this endpoint to get the authorization URL.
2. Store the `state` and `pkceVerifier` (in production, use session/Redis).
3. Redirect user to `authUrl`.
4. User authenticates with Google and is redirected to the callback URL.

---

#### GET /auth/oauth2/google/callback

Handle Google OAuth2 callback and complete authentication.

**Query Parameters:**

- `code`: Authorization code from Google.
- `state`: CSRF state token.
- `expected_state`: The state value you stored (for validation).
- `pkce_verifier`: The PKCE verifier you stored.

> [!NOTE]
> In production, `expected_state` and `pkce_verifier` should be retrieved from server-side session storage, not passed as query parameters from the client.

**Response Data:**

```json
{
  "accessToken": "jwt-access-token",
  "user": {
    "id": "uuid",
    "username": "google_12345678",
    "email": "user@gmail.com",
    "displayName": "John Doe",
    "avatarImageUrl": "https://lh3.googleusercontent.com/...",
    "isActive": true,
    "isVerified": true
  }
}
```

---

#### GET /auth/oauth2/github/login

Initiate GitHub OAuth2 authentication flow.

**Response Data:**

```json
{
  "authUrl": "https://github.com/login/oauth/authorize?...",
  "state": "csrf-state-token"
}
```

**Usage:**

1. Call this endpoint to get the authorization URL.
2. Store the `state` (in production, use session/Redis).
3. Redirect user to `authUrl`.
4. User authenticates with GitHub and is redirected to the callback URL.

---

#### GET /auth/oauth2/github/callback

Handle GitHub OAuth2 callback and complete authentication.

**Query Parameters:**

- `code`: Authorization code from GitHub.
- `state`: CSRF state token.
- `expected_state`: The state value you stored (for validation).

> [!NOTE]
> In production, `expected_state` should be retrieved from server-side session storage.

**Response Data:**

```json
{
  "accessToken": "jwt-access-token",
  "user": {
    "id": "uuid",
    "username": "githubuser",
    "email": "user@example.com",
    "displayName": "GitHub User",
    "avatarImageUrl": "https://avatars.githubusercontent.com/u/...",
    "isActive": true,
    "isVerified": true
  }
}
```
