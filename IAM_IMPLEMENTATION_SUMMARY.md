# IAM Service Implementation Summary

## Overview

This is a comprehensive Identity and Access Management (IAM) service for the kafkey-api project, implementing role-based access control (RBAC) with fine-grained permissions.

## Architecture

The implementation follows clean architecture principles with distinct layers:

### 1. Domain Layer
- **Entities**: Core business objects (User, Role, Permission, UserRole, RolePermission)
- **Repositories**: Abstract interfaces for data access
- **Services**: Domain service interfaces defining business operations

### 2. Application Layer
- **Use Cases**: Orchestrate domain services to implement specific business scenarios
- Auth use cases (login, register, token management)
- User use cases (CRUD operations, role assignment)
- Role use cases (CRUD operations, permission assignment)
- Permission use cases (CRUD operations, access checks)

### 3. Infrastructure Layer
- **Repository Implementations**: PostgreSQL implementations using Diesel ORM
- **Service Implementations**: Concrete implementations of domain services
- **Database**: Migrations for setting up IAM tables

### 4. API Layer
- **Handlers**: Axum route handlers for HTTP endpoints
- **Middleware**: Authentication and authorization middleware
- **Routes**: Endpoint definitions grouped by resource

## Database Schema

### Tables
1. **users** - Store user accounts and credentials
2. **roles** - Define user roles
3. **permissions** - Define fine-grained permissions
4. **user_roles** - Many-to-many mapping between users and roles
5. **role_permissions** - Many-to-many mapping between roles and permissions

## Key Features

### Authentication
- User registration with password hashing (bcrypt)
- JWT token-based authentication
- Token refresh mechanism
- Secure password storage and verification

### Authorization
- Role-based access control (RBAC)
- Resource-action permission model
- Hierarchical permissions through roles
- Middleware for route protection

### User Management
- CRUD operations for users
- Role assignment and revocation
- Profile management
- User status management

### Role Management
- CRUD operations for roles
- Permission assignment to roles
- Role hierarchy support
- Flexible role definitions

### Permission Management
- CRUD operations for permissions
- Resource-action model
- Fine-grained access control
- Permission checking at service level

## API Endpoints

### Authentication (`/api/v1/iam/auth`)
- `POST /login` - User login
- `POST /register` - User registration
- `POST /refresh` - Refresh JWT token
- `POST /logout` - User logout
- `POST /validate` - Validate JWT token

### Users (`/api/v1/iam/users`)
- `GET /` - List users (public)
- `POST /` - Create user (public)
- `GET /{id}` - Get user by ID (public)
- `GET /username/{username}` - Get user by username (public)
- `GET /email/{email}` - Get user by email (public)
- `PUT /{id}` - Update user (protected)
- `DELETE /{id}` - Delete user (protected)
- `POST /{id}/roles` - Assign role to user (protected)
- `DELETE /{id}/roles` - Revoke role from user (protected)
- `GET /{id}/roles` - Get user's roles (protected)
- `GET /{id}/permissions` - Get user's permissions (protected)
- `POST /{id}/permissions/check` - Check if user has permission (protected)

### Roles (`/api/v1/iam/roles`)
- `GET /` - List roles (public)
- `POST /` - Create role (protected)
- `GET /{id}` - Get role by ID (public)
- `GET /name/{name}` - Get role by name (public)
- `PUT /{id}` - Update role (protected)
- `DELETE /{id}` - Delete role (protected)
- `GET /{id}/permissions` - Get role's permissions (public)
- `POST /{id}/permissions` - Assign permission to role (protected)
- `DELETE /{id}/permissions` - Revoke permission from role (protected)
- `POST /{id}/permissions/assign` - Assign permission directly to user (protected)

### Permissions (`/api/v1/iam/permissions`)
- `GET /` - List permissions (public)
- `POST /` - Create permission (protected)
- `GET /{id}` - Get permission by ID (public)
- `GET /name/{name}` - Get permission by name (public)
- `GET /resource/{resource}/action/{action}` - Get permission by resource and action (public)
- `PUT /{id}` - Update permission (protected)
- `DELETE /{id}` - Delete permission (protected)
- `GET /resources` - Get all resources (public)
- `GET /resources/{resource}/actions` - Get actions for a resource (public)
- `GET /users/{user_id}` - Get user's permissions (public)
- `GET /roles/{role_id}` - Get role's permissions (public)
- `POST /users/{user_id}/check` - Check if user has permission (protected)

## Implementation Highlights

### Security Features
- Password hashing with bcrypt
- JWT-based authentication with configurable expiration
- Role-based access control
- Resource-action permission model
- Authentication middleware for protected routes
- Permission checking middleware

### Architecture Patterns
- Clean architecture with clear separation of concerns
- Dependency injection pattern
- Repository pattern for data access
- Service layer for business logic
- Use case pattern for application logic

### Error Handling
- Centralized error handling
- Proper HTTP status codes
- Logging for debugging and monitoring

## Usage Examples

### Basic Authentication Flow
1. Register a new user with password
2. Login with credentials to get JWT token
3. Use token for accessing protected endpoints
4. Refresh token when needed

### Authorization Flow
1. Assign role(s) to user
2. Assign permission(s) to role(s)
3. Check user permissions before accessing resources
4. Use middleware to protect routes

## Setup Instructions

1. Set up PostgreSQL database
2. Configure environment variables (DATABASE_URL, JWT_SECRET)
3. Run database migrations
4. Start the application
5. Use API endpoints to manage IAM

This IAM service provides a robust foundation for implementing secure authentication and authorization in the kafkey-api application.