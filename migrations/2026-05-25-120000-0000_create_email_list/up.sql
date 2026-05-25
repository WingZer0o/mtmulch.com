CREATE TABLE email_list (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR NOT NULL UNIQUE,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_public_key VARCHAR,
    unsubscribe_public_key VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
