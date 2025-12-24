# IAM Service Implementation

This project implements a comprehensive Identity and Access Management (IAM) service with role-based access control for the kafkey-api application.

## Architecture

The IAM service follows clean architecture principles with distinct layers:

### Domain Layer
Contains core business entities and interfaces:
- **Entities**: User, Role, Permission, UserRole, RolePermission
- **Repositories**: Interfaces for data access
- **Services**: Domain service interfaces for business logic

### Application Layer
Contains use cases that orchestrate domain services:
- Auth use cases: Login, Register, Token management
- User use cases: CRUD operations, role assignment
- Role use cases: CRUD operations, permission assignment
- Permission use cases: CRUD operations, access checks

### Infrastructure Layer
Contains implementations for external dependencies:
- Database repositories using Diesel ORM
- Authentication using JWT tokens
- Password hashing with bcrypt

### API Layer
HTTP endpoints using Axum framework:
- Authentication endpoints
- User management endpoints
- Role management endpoints
- Permission management endpoints

## Database Schema

### Users Table
- id (PK, INT)
- username (VARCHAR, UNIQUE)
- email (VARCHAR, UNIQUE)
- password_hash (VARCHAR)
- first_name (VARCHAR, nullable)
- last_name (VARCHAR, nullable)
- active (BOOLEAN)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)

### Roles Table
- id (PK, INT)
- name (VARCHAR, UNIQUE)
- description (TEXT, nullable)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)

### Permissions Table
- id (PK, INT)
- name (VARCHAR, UNIQUE)
- resource (VARCHAR)
- action (VARCHAR)
- description (TEXT, nullable)
- created_at (TIMESTAMP)
- updated_at (TIMESTAMP)

### User_Roles Table (Many-to-Many)
- id (PK, INT)
- user_id (FK to users)
- role_id (FK to roles)
- created_at (TIMESTAMP)

### Role_Permissions Table (Many-to-Many)
- id (PK, INT)
- role_id (FK to roles)
- permission_id (FK to permissions)
- created_at (TIMESTAMP)

## Key Features

### Authentication
- User registration and login
- JWT token generation and validation
- Password hashing and verification
- Token refresh mechanism

### Authorization
- Role-based access control (RBAC)
- Resource-action permission model
- Hierarchical permissions through roles
- Permission checking at service level

### User Management
- CRUD operations for users
- Role assignment and revocation
- User status management
- Profile updates

### Role Management
- CRUD operations for roles
- Permission assignment to roles
- Role hierarchy support

### Permission Management
- CRUD operations for permissions
- Resource-action model
- Fine-grained access control

## API Endpoints

### Authentication
- POST `/api/v1/iam/auth/login` - User login
- POST `/api/v1/iam/auth/register` - User registration
- POST `/api/v1/iam/auth/refresh` - Refresh JWT token
- POST `/api/v1/iam/auth/logout` - User logout
- POST `/api/v1/iam/auth/validate` - Validate JWT token

### Users
- GET `/api/v1/iam/users` - List users
- POST `/api/v1/iam/users` - Create user
- GET `/api/v1/iam/users/{id}` - Get user by ID
- GET `/api/v1/iam/users/username/{username}` - Get user by username
- GET `/api/v1/iam/users/email/{email}` - Get user by email
- PUT `/api/v1/iam/users/{id}` - Update user
- DELETE `/api/v1/iam/users/{id}` - Delete user
- POST `/api/v1/iam/users/{id}/roles` - Assign role to user
- DELETE `/api/v1/iam/users/{id}/roles` - Revoke role from user
- GET `/api/v1/iam/users/{id}/roles` - Get user's roles
- GET `/api/v1/iam/users/{id}/permissions` - Get user's permissions
- POST `/api/v1/iam/users/{id}/permissions/check` - Check if user has permission

### Roles
- GET `/api/v1/iam/roles` - List roles
- POST `/api/v1/iam/roles` - Create role
- GET `/api/v1/iam/roles/{id}` - Get role by ID
- GET `/api/v1/iam/roles/name/{name}` - Get role by name
- PUT `/api/v1/iam/roles/{id}` - Update role
- DELETE `/api/v1/iam/roles/{id}` - Delete role
- GET `/api/v1/iam/roles/{id}/permissions` - Get role's permissions
- POST `/api/v1/iam/roles/{id}/permissions` - Assign permission to role
- DELETE `/api/v1/iam/roles/{id}/permissions` - Revoke permission from role
- POST `/api/v1/iam/roles/{id}/permissions/assign` - Assign permission directly to user

### Permissions
- GET `/api/v1/iam/permissions` - List permissions
- POST `/api/v1/iam/permissions` - Create permission
- GET `/api/v1/iam/permissions/{id}` - Get permission by ID
- GET `/api/v1/iam/permissions/name/{name}` - Get permission by name
- GET `/api/v1/iam/permissions/resource/{resource}/action/{action}` - Get permission by resource and action
- PUT `/api/v1/iam/permissions/{id}` - Update permission
- DELETE `/api/v1/iam/permissions/{id}` - Delete permission
- GET `/api/v1/iam/permissions/resources` - Get all resources
- GET `/api/v1/iam/permissions/resources/{resource}/actions` - Get actions for resource
- GET `/api/v1/iam/permissions/users/{user_id}` - Get user's permissions
- GET `/api/v1/iam/permissions/roles/{role_id}` - Get role's permissions
- POST `/api/v1/iam/permissions/users/{user_id}/check` - Check if user has permission

## Security Features

### Authentication
- JWT-based authentication
- Secure password storage with bcrypt
- Token expiration handling
- Refresh token mechanism

### Authorization
- Role-based access control (RBAC)
- Resource-action permission model
- Granular permission checks
- Middleware for route protection

### Input Validation
- Request validation at handler level
- Error handling and logging
- Secure error responses

## Getting Started

### Database Setup
1. Set up PostgreSQL database
2. Create a `.env` file with database URL
3. Run migrations:
   ```bash
   diesel migration run
   ```

### Configuration
1. Copy `example.env` to `.env`
2. Update configuration values:
   - DATABASE_URL
   - JWT_SECRET
   - JWT_EXPIRATION_HOURS

### Running the Application
```bash
cargo run
```

## Usage Examples

### Register a New User
```bash
curl -X POST http://localhost:4000/api/v1/iam/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "securepassword123",
    "first_name": "John",
    "last_name": "Doe"
  }'
```

### Login
```bash
curl -X POST http://localhost:4000/api/v1/iam/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "password": "securepassword123"
  }'
```

### Create a Role
```bash
curl -X POST http://localhost:4000/api/v1/iam/roles \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "name": "admin",
    "description": "Administrator with full access"
  }'
```

### Assign Role to User
```bash
curl -X POST http://localhost:4000/api/v1/iam/users/1/roles \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "role_id": 1
  }'
```

### Check User Permission
```bash
curl -X POST http://localhost:4000/api/v1/iam/permissions/users/1/check \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "resource": "users",
    "action": "read"
  }'
```

## Implementation Details

### Custom Middleware
- Authentication middleware for protected routes
- Permission checking middleware for fine-grained access control
- Error handling and response formatting

### Service Layer
- Separation of concerns between repositories and handlers
- Business logic encapsulation
- Dependency injection pattern

### Database Layer
- Diesel ORM for type-safe database queries
- Connection pooling with diesel-async
- Migration management

## Future Enhancements

1. Multi-tenancy support
2. OAuth2 integration
3. Password reset functionality
4. Two-factor authentication
5. Audit logging
6. Permission caching
7. UI dashboard for management
8. Advanced RBAC features