-- Add up migration script here
CREATE TABLE IF NOT EXISTS daily_revenue (
    id BIGSERIAL PRIMARY KEY,
    worker_name VARCHAR NOT NULL,
    revenue BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);
