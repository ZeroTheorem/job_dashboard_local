-- Add up migration script here
CREATE INDEX IF NOT EXISTS worker_name_created_at_idx ON daily_revenue(worker_name, created_at);
