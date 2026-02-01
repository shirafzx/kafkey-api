-- Index for token cleanup job (range query)
CREATE INDEX IF NOT EXISTS idx_blacklisted_tokens_expires_at ON blacklisted_tokens(expires_at);

-- Indexes for token lookups (exact match)
CREATE INDEX IF NOT EXISTS idx_users_verification_token ON users(verification_token);
CREATE INDEX IF NOT EXISTS idx_users_password_reset_token ON users(password_reset_token);

-- Index for sorting users
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
