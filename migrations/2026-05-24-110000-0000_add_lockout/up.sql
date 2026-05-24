ALTER TABLE users
    ADD COLUMN is_locked BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN unlock_public_key VARCHAR;

CREATE TABLE failed_login_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_failed_login_attempts_user_id ON failed_login_attempts(user_id);
