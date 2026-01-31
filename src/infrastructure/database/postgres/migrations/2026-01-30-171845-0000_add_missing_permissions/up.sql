-- Add missing permissions
INSERT INTO permissions (name, resource, action, description) VALUES
    ('permissions.update', 'permissions', 'update', 'Update permission information'),
    ('permissions.delete', 'permissions', 'delete', 'Delete permissions');

-- Assign these new permissions to admin role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id
FROM roles r
CROSS JOIN permissions p
WHERE r.name = 'admin'
  AND p.name IN ('permissions.update', 'permissions.delete');
