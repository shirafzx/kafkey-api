-- Add new fields to users table for enhanced authentication
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS email VARCHAR(100) UNIQUE NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT true,
    ADD COLUMN IF NOT EXISTS is_verified BOOLEAN DEFAULT false,
    ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ;

-- Remove the default constraint after adding the column
ALTER TABLE users ALTER COLUMN email DROP DEFAULT;
