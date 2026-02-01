---

## Error Codes Reference

The API uses standardized error codes in the `code` field of error responses. Here are the common codes you may encounter:

### Authentication Errors

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_CREDENTIALS` | 401 | Username/email or password is incorrect |
| `ACCOUNT_LOCKED` | 403 | Account has been locked due to too many failed login attempts |
| `EMAIL_NOT_VERIFIED` | 403 | Email verification is required before this action |
| `INVALID_TOKEN` | 401 | JWT token is invalid, expired, or malformed |
| `TOKEN_BLACKLISTED` | 401 | Token has been revoked/blacklisted |
| `MFA_REQUIRED` | 200 | 2FA verification is required to complete login |

### Validation Errors

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Input validation failed (see `errors` array for details) |
| `DUPLICATE_USERNAME` | 409 | Username is already taken |
| `DUPLICATE_EMAIL` | 409 | Email address is already registered |

### Authorization Errors

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Authentication is required |
| `FORBIDDEN` | 403 | User lacks required permissions |
| `CSRF_TOKEN_INVALID` | 403 | CSRF token is missing or invalid |

### Resource Errors

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `NOT_FOUND` | 404 | Requested resource does not exist |
| `RESOURCE_NOT_FOUND` | 404 | Specific resource (user, role, permission) not found |

### Rate Limiting

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests, check `Retry-After` header |

### Server Errors

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INTERNAL_SERVER_ERROR` | 500 | An unexpected server error occurred |
| `SERVICE_UNAVAILABLE` | 503 | Service is temporarily unavailable |
| `REQUEST_TIMEOUT` | 408 | Request took too long to process |

---

## Testing

The API includes comprehensive unit test coverage for all business logic use cases:

### Running Tests

```bash
# Run all tests
cargo test

# Run only use case unit tests
cargo test --lib use_cases::tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_register_with_unique_credentials
```

### Test Coverage

- **Audit Use Cases**: 3/3 tests ✅
- **Authentication Use Cases**: 3/3 tests ✅
- **Permissions Use Cases**: 6/6 tests ✅
- **Roles Use Cases**: 8/8 tests ✅
- **Users Use Cases**: 12/12 tests ✅
- **OAuth2 Use Cases**: 3/3 tests ✅

**Total: 34 tests with 100% pass rate**

### Test Structure

All unit tests use mocked repositories (via `mockall`) to isolate business logic from database dependencies. Integration tests can be run against a test database.

---

## Additional Features

### Background Jobs

The API automatically runs background cleanup tasks:

- **Token Blacklist Cleanup**: Runs every hour to remove expired blacklisted tokens from the database

### Security Features

- **JWT Authentication**: Access and refresh token support
- **CSRF Protection**: Double-submit cookie pattern for state-changing requests
- **Rate Limiting**: 10 requests per minute per IP address
- **Password Hashing**: Uses Argon2id for secure password storage
- **Account Lockout**: Automatic lockout after 5 failed login attempts
- **2FA Support**: TOTP-based two-factor authentication with backup codes
- **OAuth2 Security**: State validation and PKCE for enhanced security

### Monitoring & Observability

- **Request IDs**: Each request tracked with unique UUID
- **Prometheus Metrics**: Built-in metrics for monitoring
- **Sentry Integration**: Error tracking and performance monitoring
- **Structured Logging**: JSON-formatted logs with context
- **Audit Logging**: MongoDB-based audit trail for all actions

### Technical Stack

- **Framework**: Axum (Rust async web framework)
- **Database**: PostgreSQL (primary data store)
- **Audit Store**: MongoDB (audit logs)
- **Authentication**: JWT with RS256 signing
- **Password Hashing**: Argon2id
- **OAuth2**: Google and GitHub providers
- **Monitoring**: Prometheus + Sentry

---

## API Versioning

The current API version is `v1`. All endpoints are prefixed with `/api/v1`.

Future breaking changes will be introduced in new versions (e.g., `/api/v2`) while maintaining backward compatibility for at least one major version.

---

## Support & Feedback

For questions, issues, or feature requests, please refer to the project repository or contact the development team.

**Last Updated**: February 2026
