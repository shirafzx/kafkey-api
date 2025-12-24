# IAM Service Implementation

This document provides an overview of the Identity and Access Management (IAM) system implemented for the kafkey-api project.

## Overview

The IAM service provides a comprehensive role-based access control (RBAC) system with the following key components:

### 1. Authentication
- JWT-based authentication
- Password hashing with bcrypt
- Token generation and validation
- Login and registration endpoints

### 2. Authorization
- Resource-action permission model
- Role-based access control (RBAC)
- Fine-grained permission checking
- Middleware for route protection

### 3. User Management
- User CRUD operations
- Role assignment to users
- User status management

### 4. Role Management
- Role CRUD operations
- Permission assignment to roles
- Hierarchical role definitions

### 5. Permission Management
- Permission CRUD operations
- Resource-action model
- Permission inheritance through roles

## Database Schema

### Tables

1. **users** - Store user credentials and profile information
   - id (PK, INT)
   - username (VARCHAR, UNIQUE)
   - email (VARCHAR, UNIQUE)
   - password_hash (VARCHAR)
   - first_name (VARCHAR, nullable)
   - last_name (VARCHAR, nullable)
   - active (BOOLEAN)
   - created_at (TIMESTAMP)
   - updated_at (TIMESTAMP)

2. **roles** - Define user roles
   - id (PK, INT)
   - name (VARCHAR, UNIQUE)
   - description (TEXT, nullable)
   - created_at (TIMESTAMP)
   - updated_at (TIMESTAMP)

3. **permissions** - Define fine-grained permissions
   - id (PK, INT)
   - name (VARCHAR, UNIQUE)
   - resource (VARCHAR)
   - action (VARCHAR)
   - description (TEXT, nullable)
   - created_at (TIMESTAMP)
   - updated_at (TIMESTAMP)

4. **user_roles** - Many-to-many relationship between users and roles
   - id (PK, INT)
   - user_id (FK to users)
   - role_id (FK to roles)
   - created_at (TIMESTAMP)

5. **role_permissions** - Many-to-many relationship between roles and permissions
   - id (PK, INT)
   - role_id (FK to roles)
   - permission_id (FK to permissions)
   - created_at (TIMESTAMP)

## API Endpoints

### Authentication
- `POST /api/v1/iam/auth/login` - User login
- `POST /api/v1/iam/auth/register` - User registration
- `POST /api/v1/iam/auth/refresh` - Refresh JWT token
- `POST /api/v1/iam/auth/logout` - User logout
- `POST /api/v1/iam/auth/validate` - Validate JWT token

### Users
- `GET /api/v1/iam/users` - List users
- `POST /api/v1/iam/users` - Create user
- `GET /api/v1/iam/users/{id}` - Get user by ID
- `GET /api/v1/iam/users/username/{username}` - Get user by username
- `GET /api/v1/iam/users/email/{email}` - Get user by email
- `PUT /api/v1/iam/users/{id}` - Update user
- `DELETE /api/v1/iam/users/{id}` - Delete user
- `POST /api/v1/iam/users/{id}/roles` - Assign role to user
- `DELETE /api/v1/iam/users/{id}/roles` - Revoke role from user
- `GET /api/v1/iam/users/{id}/roles` - Get user's roles
- `GET /api/v1/iam/users/{id}/permissions` - Get user's permissions
- `POST /api/v1/iam/users/{id}/permissions/check` - Check if user has permission

### Roles
- `GET /api/v1/iam/roles` - List roles
- `POST /api/v1/iam/roles` - Create role
- `GET /api/v1/iam/roles/{id}` - Get role by ID
- `GET /api/v1/iam/roles/name/{name}` - Get role by name
- `PUT /api/v1/iam/roles/{id}` - Update role
- `DELETE /api/v1/iam/roles/{id}` - Delete role
- `GET /api/v1/iam/roles/{id}/permissions` - Get role's permissions
- `POST /api/v1/iam/roles/{id}/permissions` - Assign permission to role
- `DELETE /api/v1/iam/roles/{id}/permissions` - Revoke permission from role
- `POST /api/v1/iam/roles/{id}/permissions/assign` - Assign permission directly to user

### Permissions
- `GET /api/v1/iam/permissions` - List permissions
- `POST /api/v1/iam/permissions` - Create permission
- `GET /api/v1/iam/permissions/{id}` - Get permission by ID
- `GET /api/v1/iam/permissions/name/{name}` - Get permission by name
- `GET /api/v1/iam/permissions/resource/{resource}/action/{action}` - Get permission by resource and action
- `PUT /api/v1/iam/permissions/{id}` - Update permission
- `DELETE /api/v1/iam/permissions/{id}` - Delete permission
- `GET /api/v1/iam/permissions/resources` - Get all resources
- `GET /api/v1/iam/permissions/resources/{resource}/actions` - Get actions for a resource
- `GET /api/v1/iam/permissions/users/{user_id}` - Get user's permissions
- `GET /api/v1/iam/permissions/roles/{role_id}` - Get role's permissions
- `POST /api/v1/iam/permissions/users/{user_id}/check` - Check if user has permission

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

## Security Features

### Authentication
- JWT-based authentication with configurable expiration
- Secure password storage with bcrypt
- Token refresh mechanism
- Protection against brute force attacks

### Authorization
- Role-based access control (RBAC)
- Resource-action permission model
- Fine-grained permission checking
- Middleware for route protection

## Implementation Details

The IAM service follows clean architecture principles:

1. **Domain Layer**: Core business objects and interfaces
2. **Application Layer**: Use cases that orchestrate domain services
3. **Infrastructure Layer**: Database implementations and external dependencies
4. **API Layer**: HTTP endpoints and middleware

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

## Default Setup

The system includes default roles and permissions that you can customize:

### Default Roles
- **admin**: Full access to all resources
- **user_manager**: Can manage users and assign roles
- **role_manager**: Can manage roles and permissions
- **regular_user**: Limited access to specific resources

### Default Permissions
- **user:create, user:read, user:update, user:delete**
- **role:create, role:read, role:update, role:delete**
- **permission:create, permission:read, permission:update, permission:delete**
- **content:create, content:read, content:update, content:delete**

This IAM service provides a robust foundation for implementing secure authentication and authorization in your kafkey-api application.