-- Create IAM tables for role-based and permission-based access control

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Permissions table
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Role-Permission join table (many-to-many)
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (role_id, permission_id)
);

-- User-Role join table (many-to-many)
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Insert default roles
INSERT INTO roles (name, description) VALUES
    ('admin', 'Administrator with full system access'),
    ('user', 'Standard user with basic permissions');

-- Insert default permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('users.read', 'users', 'read', 'Read user information'),
    ('users.create', 'users', 'create', 'Create new users'),
    ('users.update', 'users', 'update', 'Update user information'),
    ('users.delete', 'users', 'delete', 'Delete users'),
    ('roles.read', 'roles', 'read', 'Read role information'),
    ('roles.create', 'roles', 'create', 'Create new roles'),
    ('roles.update', 'roles', 'update', 'Update role information'),
    ('roles.delete', 'roles', 'delete', 'Delete roles'),
    ('permissions.read', 'permissions', 'read', 'Read permission information'),
    ('permissions.create', 'permissions', 'create', 'Create new permissions');

-- Assign all permissions to admin role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'admin';

-- Assign basic permissions to user role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'user'
  AND p.name IN ('users.read');

-- Create indexes for better query performance
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);
CREATE INDEX idx_permissions_resource_action ON permissions(resource, action);
