# Kafkey API - Development Tasks

## ✅ Completed Tasks

### Phase 1: Database Schema Design

- [x] Create migration for roles table
- [x] Create migration for permissions table
- [x] Create migration for role_permissions join table
- [x] Create migration for user_roles join table
- [x] Modify users table migration to support IAM features

### Phase 2: Domain Layer

- [x] Create Role entity
- [x] Create Permission entity
- [x] Update User entity to support roles
- [x] Create RoleRepository trait
- [x] Create PermissionRepository trait
- [x] Update UserRepository trait

### Phase 3: Infrastructure Layer

- [x] Implement PostgreSQL RoleRepository
- [x] Implement PostgreSQL PermissionRepository
- [x] Update PostgreSQL UserRepository for role queries
- [x] Update Diesel schema with new tables

### Phase 4: Application Layer - Use Cases

- [x] Create authentication use cases (login, refresh token)
- [x] Update user use cases for authentication with roles
- [x] Create password hashing service (using argon2)

### Phase 5: Authentication & Authorization

- [x] Create authentication service (generate/validate JWT)
- [x] Create authentication middleware to extract JWT
- [x] Create authorization middleware (PBAC check)
- [x] Add granular `RequirePermission` guards

### Phase 6: API Layer - Managed Routes

- [x] `POST /api/v1/auth/sign-up` - Register with default role
- [x] `POST /api/v1/auth/login` - Login with credentials
- [x] `POST /api/v1/auth/refresh` - Refresh access token
- [x] `GET /api/v1/users/me` - Get current user profile
- [x] `PUT /api/v1/users/me` - Update current user profile
- [x] `GET /api/v1/users` - List all users (Paginated, requires `admin` role and `users.read`)
- [x] `GET /api/v1/users/:id` - Get user by ID (requires `users.read`)
- [x] `PUT /api/v1/users/:id` - Update user (requires `users.update`)
- [x] `DELETE /api/v1/users/:id` - Delete user (requires `users.delete`)
- [x] `POST /api/v1/users/:id/roles` - Assign role to user (requires `users.update`)
- [x] `DELETE /api/v1/users/:id/roles/:roleId` - Remove role from user (requires `users.update`)
- [x] `GET /roles` - CRUD for roles (requires `roles.*`)
- [x] `GET /permissions` - CRUD for permissions (requires `permissions.*`)

### Phase 7: API Standardization & Refactoring

- [x] Standardize API response envelope (`ApiResponse`, `ApiErrorResponse`)
- [x] Enforce `camelCase` for all JSON keys globally
- [x] Implement `x-request-id` tracking middleware
- [x] Refactor DTOs into domain-specific modules in the application layer
- [x] Refactor pagination to use nested structure with `hasNext`/`hasPrev` flags

### Phase 8: Documentation

- [x] Create README.md with installation instructions
- [x] Create initial API documentation
- [x] Create technical walkthrough
- [x] Create database schema documentation

### Phase 9: Security Enhancements

- [x] Add global rate limiting for all endpoints
- [x] Implement token blacklist for logout functionality
- [x] Add account lockout after failed login attempts
- [x] Implement email verification flow
- [x] Add password reset functionality
- [x] Add two-factor authentication (2FA) support
- [x] Implement CSRF protection
- [x] Implement Request Validation

### Phase 10: Performance & Optimization

- [x] Add database query optimization and indexes
- [x] Implement caching for frequently accessed data (roles, permissions)
- [x] Add database connection pool tuning
- [x] Implement pagination for list endpoints (Users)
- [x] Add query result limits

### Phase 11: Monitoring & Logging

- [x] Add structured logging for authentication events
- [x] Implement audit logging for admin actions
- [x] Add metrics collection (Prometheus/OpenTelemetry)
- [x] Set up health check endpoints with database connectivity
- [x] Add error tracking integration (Sentry, etc.)

### Phase 12: Persistent Auditing (MongoDB)

- [x] Integrate MongoDB client and connection pooling
- [x] Implement persistent audit logging for administrative actions
- [x] Standardize UUID storage using BSON Subtype 4
- [x] Refactor MongoDB infrastructure for modularity

### Phase 13: Containerization & Deployment

- [x] Create multi-stage `Dockerfile` for efficient builds
- [x] Create `docker-compose.yml` with Postgres 17, MongoDB 8.0, and Redis
- [x] Implement standardized networking between service containers
- [x] Set up volume persistence for database data

### Phase 14: Continuous Integration & Deployment

- [x] Create GitHub Actions CI workflow for automated validation
- [x] Implement `cargo fmt`, `clippy`, and `cargo test` in CI pipeline
- [x] Implement Docker image build check in CI using Buildx
- [x] Optimize CI performance with `rust-cache` and Docker layer caching

### Phase 15: OAuth2 Integration

- [x] Implement OAuth2 authentication with Google and GitHub
- [x] Create database schema for social accounts (`user_social_accounts` table)
- [x] Implement OAuth2 domain entities and repository traits
- [x] Create OAuth2Service using `oauth2` crate with PKCE support
- [x] Implement secure account linking with email verification checks
- [x] Create API routes for OAuth2 login and callbacks
- [x] Add environment configuration for OAuth2 credentials

---

## 🚧 In Progress / TODO

- [ ] Multi-tenancy support
- [ ] WebSocket support for real-time notifications

---

## 🎯 Future Enhancements

- [ ] Nothing Now

---

## 📝 Notes

- Default roles: `admin`, `user`
- Password hashing: Argon2
- Response Format: Standard envelope with `camelCase` keys
- Global header: `x-request-id` included in all responses
