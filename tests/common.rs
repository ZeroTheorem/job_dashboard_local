use axum::Router;
use dotenv::dotenv;
use job_dashboard_backend::{create_app_with_state, database, state::AppState};
use sqlx::PgPool;

pub fn setup(pool: PgPool) -> (Router, PgPool) {
    dotenv().ok();

    let database = database::Database { pool: pool.clone() };
    let state = AppState { database: database };
    let app = create_app_with_state(state);
    (app, pool)
}
