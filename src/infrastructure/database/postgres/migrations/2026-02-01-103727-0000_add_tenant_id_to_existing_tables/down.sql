-- Remove indexes
DROP INDEX IF EXISTS idx_permissions_tenant_id;
DROP INDEX IF EXISTS idx_roles_tenant_id;
DROP INDEX IF EXISTS idx_users_tenant_id;

-- Remove unique constraint on tenant+email
DROP INDEX IF EXISTS idx_users_tenant_email;

-- Restore global email uniqueness
ALTER TABLE users ADD CONSTRAINT users_email_key UNIQUE (email);

-- Remove tenant_id columns
ALTER TABLE permissions DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE roles DROP COLUMN IF EXISTS tenant_id;
ALTER TABLE users DROP COLUMN IF EXISTS tenant_id;
