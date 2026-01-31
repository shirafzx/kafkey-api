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

| Header          | Description                                                 |
| --------------- | ----------------------------------------------------------- |
| `Authorization` | `Bearer <access_token>` (required for protected routes)     |
| `x-request-id`  | Unique ID for tracking requests (provided in all responses) |

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
  "code": "ERROR_CODE",
  "message": "Human readable error message",
  "errors": [
    {
      "field": "fieldName",
      "reason": "Why the field failed validation"
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

## Endpoints

### Authentication

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

### User Management

#### GET /users/me

Get current authenticated user's profile.

#### PUT /users/me

Update current authenticated user's profile.

**Request Body:**

```json
{
  "displayName": "New Name",
  "avatarImageUrl": "https://newurl.com/img.png"
}
```

#### GET /users (Admin Only)

List all users with pagination.
**Params:** `page` (default 1), `pageSize` (default 20)

#### GET /users/{id} (Admin Only)

Get details of a specific user.

#### PUT /users/{id} (Admin Only)

Update user details.

#### DELETE /users/{id} (Admin Only)

Delete a user.

---

### IAM Management (Admin Only)

- Roles: `GET/POST /roles`, `GET/PUT/DELETE /roles/{id}`
- Permissions: `GET/POST /permissions`, `GET/PUT/DELETE /permissions/{id}`
- Role Assignment: `POST /users/{id}/roles`, `DELETE /users/{id}/roles/{roleId}`
- Permission Assignment: `POST /roles/{id}/permissions`

---

### Health Check

**GET /health-check**
Returns a `200 OK` with system health status.
