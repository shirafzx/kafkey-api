-- Remove 2FA fields from users table
ALTER TABLE users
    DROP COLUMN IF EXISTS two_factor_secret,
    DROP COLUMN IF EXISTS two_factor_enabled,
    DROP COLUMN IF EXISTS two_factor_backup_codes;
