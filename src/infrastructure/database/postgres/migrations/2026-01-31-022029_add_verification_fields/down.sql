-- Remove verification fields from users table
ALTER TABLE users
    DROP COLUMN IF EXISTS verification_token,
    DROP COLUMN IF EXISTS verification_token_expires_at;
