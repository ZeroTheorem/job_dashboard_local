-- Add up migration script here
CREATE TABLE IF NOT EXISTS plan (
    id BIGSERIAL PRIMARY KEY,
    value BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
