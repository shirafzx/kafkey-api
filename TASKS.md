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

- [x] Add JWT dependencies (jsonwebtoken, argon2)
- [x] Create JWT token service (generate, validate)
- [x] Create password hashing service (using argon2)

### Phase 6: API Layer

- [x] Create authentication routes (login, refresh token)
- [x] Update registration to assign default role
- [x] Add `/api/v1` prefix to all routes
- [x] Create user profile routes (`GET/PUT /api/v1/users/me`)
- [x] Create administrative user list route (`GET /api/v1/users`)

### Phase 7: Documentation

- [x] Create README.md with installation instructions
- [x] Create API documentation
- [x] Create implementation plan documentation
- [x] Create walkthrough documentation

---

## 🚧 In Progress / TODO

### Authentication & Authorization

- [x] Create authentication middleware to extract JWT from requests
- [x] Create authorization middleware (check roles/permissions)
- [x] Add `RequireRole` guard for role-based access
- [x] Add `RequirePermission` guard for permission-based access

### API Layer - Additional Routes

### User Management

- [x] `GET /api/v1/users/me` - Get current user profile
- [x] `PUT /api/v1/users/me` - Update current user profile
- [x] `GET /api/v1/users/:id` - Get user by ID (admin only)
- [x] `PUT /api/v1/users/:id` - Update user (admin only)
- [x] `DELETE /api/v1/users/:id` - Delete user (admin only)
- [x] `POST /api/v1/users/:id/roles` - Assign role to user (admin only)
- [x] `DELETE /api/v1/users/:id/roles/:roleId` - Remove role from user (admin only)
- [x] `GET /api/v1/users/:id/roles` - Get user's roles
- [x] `GET /api/v1/users/:id/permissions` - Get user's permissions

#### Role Management

- [x] `GET /api/v1/roles` - List all roles
- [x] `POST /api/v1/roles` - Create new role (admin only)
- [x] `GET /api/v1/roles/:id` - Get role details
- [x] `PUT /api/v1/roles/:id` - Update role (admin only)
- [x] `DELETE /api/v1/roles/:id` - Delete role (admin only)
- [x] `GET /api/v1/roles/:id/permissions` - Get role's permissions
- [x] `POST /api/v1/roles/:id/permissions` - Assign permission to role (admin only)
- [x] `DELETE /api/v1/roles/:id/permissions/:permissionId` - Remove permission from role (admin only)

#### Permission Management

- [x] `GET /api/v1/permissions` - List all permissions
- [x] `POST /api/v1/permissions` - Create new permission (admin only)
- [x] `GET /api/v1/permissions/:id` - Get permission details
- [x] `PUT /api/v1/permissions/:id` - Update permission (admin only)
- [x] `DELETE /api/v1/permissions/:id` - Delete permission (admin only)

### Testing

- [x] Test role-based access control in routes
- [x] Test permission-based access control in routes

### Security Enhancements

- [x] Add global rate limiting for all endpoints
- [x] Implement token blacklist for logout functionality
- [ ] Add account lockout after failed login attempts
- [ ] Implement email verification flow
- [ ] Add password reset functionality
- [ ] Add two-factor authentication (2FA) support
- [ ] Implement CSRF protection
- [ ] Add request validation middleware

### Performance & Optimization

- [ ] Add database query optimization and indexes
- [ ] Implement caching for frequently accessed data (roles, permissions)
- [ ] Add database connection pool tuning
- [ ] Implement pagination for list endpoints
- [ ] Add query result limits

### Monitoring & Logging

- [ ] Add structured logging for authentication events
- [ ] Implement audit logging for admin actions
- [ ] Add metrics collection (Prometheus/OpenTelemetry)
- [ ] Set up health check endpoints with database connectivity
- [ ] Add error tracking integration (Sentry, etc.)

### DevOps & Deployment

- [ ] Create Dockerfile for containerization
- [ ] Create docker-compose.yml for local development
- [ ] Set up CI/CD pipeline (GitHub Actions/GitLab CI)
- [ ] Add environment-specific configuration
- [ ] Create deployment documentation
- [ ] Set up database backup strategy
- [ ] Configure HTTPS/TLS in production

### Code Quality

- [ ] Add clippy lints configuration
- [ ] Set up rustfmt configuration
- [ ] Add pre-commit hooks
- [ ] Improve error handling and custom error types
- [ ] Add input validation for all endpoints
- [ ] Remove unused imports and dead code
- [ ] Add comprehensive documentation comments

---

## 🎯 Future Enhancements

### Advanced Features

- [ ] OAuth2 integration (Google, GitHub, etc.)
- [ ] SAML support for enterprise SSO
- [ ] Multi-tenancy support
- [ ] API key authentication for service-to-service calls
- [ ] WebSocket support for real-time notifications
- [ ] GraphQL API endpoint
- [ ] Admin dashboard UI
- [ ] User profile image upload
- [ ] Session management and device tracking

### Database

- [ ] Add soft delete for users
- [ ] Implement database migrations rollback testing
- [ ] Add database seeding for development/testing
- [ ] Consider read replicas for scaling

---

## 📝 Notes

- Default roles created: `admin`, `user`
- Default permissions include CRUD operations for users, roles, and permissions
- Access tokens expire after 15 minutes
- Refresh tokens expire after 7 days
- Password hashing uses Argon2 algorithm
- All API endpoints use `/api/v1` prefix for versioning
