CREATE TABLE refresh_tokens (
    uuid UUID NOT NULL,
    user_uuid UUID REFERENCES users(uuid) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);
