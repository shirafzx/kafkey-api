-- Remove the added columns from users table
ALTER TABLE users
    DROP COLUMN IF EXISTS last_login_at,
    DROP COLUMN IF EXISTS is_verified,
    DROP COLUMN IF EXISTS is_active,
    DROP COLUMN IF EXISTS email;
