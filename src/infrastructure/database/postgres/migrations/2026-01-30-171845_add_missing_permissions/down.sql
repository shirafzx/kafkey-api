-- Remove the permissions (cascading into role_permissions)
DELETE FROM permissions WHERE name IN ('permissions.update', 'permissions.delete');
