-- Your SQL goes here
CREATE TABLE blacklisted_tokens (
    jti UUID PRIMARY KEY,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
