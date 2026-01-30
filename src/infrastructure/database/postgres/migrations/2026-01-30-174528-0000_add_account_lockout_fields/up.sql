-- Add account lockout fields to users table
ALTER TABLE users 
ADD COLUMN failed_login_attempts INTEGER DEFAULT 0 NOT NULL,
ADD COLUMN locked_at TIMESTAMPTZ;
