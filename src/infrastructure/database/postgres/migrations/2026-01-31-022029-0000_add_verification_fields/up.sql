-- Add verification fields to users table
ALTER TABLE users
    ADD COLUMN IF NOT EXISTS verification_token VARCHAR(255),
    ADD COLUMN IF NOT EXISTS verification_token_expires_at TIMESTAMPTZ;
