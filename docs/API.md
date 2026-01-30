# API Documentation

## Base URL

All API endpoints are prefixed with `/api/v1`.

```
Base URL: http://localhost:8080/api/v1
```

## Authentication

Most endpoints require authentication using JWT tokens in the `Authorization` header:

```
Authorization: Bearer <access_token>
```

## Response Format

### Success Response

```json
{
  "data": { ... },
  "message": "Success message"
}
```

### Error Response

```json
{
  "error": "Error message",
  "details": "Additional error details"
}
```

## Endpoints

### Authentication

#### POST /auth/sign-up

Register a new user account.

**Request Body:**

```json
{
  "username": "string (3-30 chars, required)",
  "email": "string (valid email, required)",
  "display_name": "string (1-50 chars, required)",
  "password": "string (min 8 chars, required)",
  "avatar_image_url": "string (optional)"
}
```

**Response:** `201 Created`

```json
{
  "message": "User registered successfully",
  "user_id": "uuid"
}
```

**Errors:**

- `400 Bad Request` - Invalid input data
- `409 Conflict` - Username or email already exists
- `500 Internal Server Error` - Server error

**Example:**

```bash
curl -X POST http://localhost:8080/api/v1/auth/sign-up \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "display_name": "John Doe",
    "password": "SecurePass123!"
  }'
```

---

#### POST /auth/login

Authenticate and receive JWT tokens.

**Request Body:**

```json
{
  "email_or_username": "string (email or username, required)",
  "password": "string (required)"
}
```

**Response:** `200 OK`

```json
{
  "user_id": "uuid",
  "access_token": "jwt-token",
  "refresh_token": "jwt-token"
}
```

**Errors:**

- `401 Unauthorized` - Invalid credentials or inactive account
- `500 Internal Server Error` - Server error

**Example:**

```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "john@example.com",
    "password": "SecurePass123!"
  }'
```

---

#### POST /auth/refresh

Refresh an expired access token.

**Request Body:**

```json
{
  "refresh_token": "string (required)"
}
```

**Response:** `200 OK`

```json
{
  "access_token": "jwt-token"
}
```

**Errors:**

- `401 Unauthorized` - Invalid or expired refresh token
- `500 Internal Server Error` - Server error

**Example:**

```bash
curl -X POST http://localhost:8080/api/v1/auth/refresh \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
  }'
```

---

### Health Check

#### GET /health-check

Check if the server is running.

**Response:** `200 OK`

```json
{
  "status": "healthy"
}
```

**Example:**

```bash
curl http://localhost:8080/health-check
```

---

## JWT Token Claims

### Access Token Claims

```json
{
  "sub": "user-uuid",
  "exp": 1234567890,
  "iat": 1234567890,
  "roles": ["user", "admin"],
  "permissions": ["users:read", "users:write"]
}
```

- `sub`: Subject (user ID)
- `exp`: Expiration time (Unix timestamp)
- `iat`: Issued at (Unix timestamp)
- `roles`: Array of role names assigned to user
- `permissions`: Array of permission strings

### Refresh Token Claims

```json
{
  "sub": "user-uuid",
  "exp": 1234567890,
  "iat": 1234567890,
  "roles": [],
  "permissions": []
}
```

Refresh tokens contain minimal information and are used only to obtain new access tokens.

---

## Rate Limiting

Currently not implemented. Consider adding rate limiting in production.

---

## CORS

CORS is enabled for all origins in development. Configure appropriately for production.

---

## Request/Response Examples

### Complete Registration Flow

```bash
# 1. Register
RESPONSE=$(curl -s -X POST http://localhost:8080/api/v1/auth/sign-up \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "display_name": "Alice Smith",
    "password": "SecurePass123!"
  }')

echo "Registration: $RESPONSE"

# 2. Login
TOKENS=$(curl -s -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email_or_username": "alice@example.com",
    "password": "SecurePass123!"
  }')

echo "Login: $TOKENS"

# Extract access token (using jq)
ACCESS_TOKEN=$(echo $TOKENS | jq -r '.access_token')

# 3. Use access token for authenticated requests
# (Add your protected endpoint here)

# 4. Refresh token when access token expires
REFRESH_TOKEN=$(echo $TOKENS | jq -r '.refresh_token')

NEW_ACCESS=$(curl -s -X POST http://localhost:8080/api/v1/auth/refresh \
  -H "Content-Type: application/json" \
  -d "{\"refresh_token\": \"$REFRESH_TOKEN\"}")

echo "New access token: $NEW_ACCESS"
```

---

## Error Codes

| HTTP Status | Description                                      |
| ----------- | ------------------------------------------------ |
| 200         | Success                                          |
| 201         | Created                                          |
| 400         | Bad Request - Invalid input                      |
| 401         | Unauthorized - Authentication required or failed |
| 403         | Forbidden - Insufficient permissions             |
| 404         | Not Found                                        |
| 409         | Conflict - Resource already exists               |
| 500         | Internal Server Error                            |

---

## Future Endpoints (Planned)

### User Management

- `GET /api/v1/users/me` - Get current user profile
- `PUT /api/v1/users/me` - Update current user profile
- `GET /api/v1/users/:id` - Get user by ID (admin)
- `POST /api/v1/users/:id/roles` - Assign role to user (admin)
- `DELETE /api/v1/users/:id/roles/:roleId` - Remove role from user (admin)

### Role Management

- `GET /api/v1/roles` - List all roles
- `POST /api/v1/roles` - Create new role (admin)
- `GET /api/v1/roles/:id` - Get role details
- `PUT /api/v1/roles/:id` - Update role (admin)
- `DELETE /api/v1/roles/:id` - Delete role (admin)
- `POST /api/v1/roles/:id/permissions` - Assign permission to role (admin)

### Permission Management

- `GET /api/v1/permissions` - List all permissions
- `POST /api/v1/permissions` - Create new permission (admin)
- `GET /api/v1/permissions/:id` - Get permission details
- `PUT /api/v1/permissions/:id` - Update permission (admin)
- `DELETE /api/v1/permissions/:id` - Delete permission (admin)
