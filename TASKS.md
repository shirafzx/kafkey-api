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

---

## 🚧 In Progress / TODO

### Security Enhancements

- [x] Add global rate limiting for all endpoints
- [x] Implement token blacklist for logout functionality
- [x] Add account lockout after failed login attempts
- [x] Implement email verification flow
- [x] Add password reset functionality
- [x] Add two-factor authentication (2FA) support
- [ ] Implement CSRF protection
- [ ] Add request validation middleware

### Performance & Optimization

- [ ] Add database query optimization and indexes
- [ ] Implement caching for frequently accessed data (roles, permissions)
- [ ] Add database connection pool tuning
- [x] Implement pagination for list endpoints (Users)
- [ ] Add query result limits

### Monitoring & Logging

- [ ] Add structured logging for authentication events
- [ ] Implement audit logging for admin actions
- [ ] Add metrics collection (Prometheus/OpenTelemetry)
- [x] Set up health check endpoints with database connectivity
- [ ] Add error tracking integration (Sentry, etc.)

### DevOps & Deployment

- [ ] Create Dockerfile for containerization
- [ ] Create docker-compose.yml for local development
- [ ] Set up CI/CD pipeline (GitHub Actions/GitLab CI)
- [ ] Add environment-specific configuration

---

## 🎯 Future Enhancements

- [ ] OAuth2 integration (Google, GitHub, etc.)
- [ ] Multi-tenancy support
- [ ] WebSocket support for real-time notifications
- [ ] Admin dashboard UI

---

## 📝 Notes

- Default roles: `admin`, `user`
- Password hashing: Argon2
- Response Format: Standard envelope with `camelCase` keys
- Global header: `x-request-id` included in all responses
