-- Add tenant_id to users (end users of your customers' apps)
-- Note: This will require a default tenant to be created first for existing users
ALTER TABLE users ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Make email unique per tenant instead of globally unique
ALTER TABLE users DROP CONSTRAINT users_email_key;
CREATE UNIQUE INDEX idx_users_tenant_email ON users(tenant_id, email);

-- Add tenant_id to roles and permissions
ALTER TABLE roles ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;
ALTER TABLE permissions ADD COLUMN tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE;

-- Create indexes for performance
CREATE INDEX idx_users_tenant_id ON users(tenant_id);
CREATE INDEX idx_roles_tenant_id ON roles(tenant_id);
CREATE INDEX idx_permissions_tenant_id ON permissions(tenant_id);
