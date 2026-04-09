mod database;
mod errors_responses;
mod handlers;
mod jsons;
mod salary;
mod state;
mod success_responses;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use dotenv::dotenv;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load env variables
    dotenv().ok();

    let state = state::AppState::builder().await?;

    let app = Router::new()
        .route("/records", post(handlers::create_record_handler))
        .route("/dashboard", get(handlers::get_dashboard))
        .route("/records/{id}", delete(handlers::delete_record))
        .route("/records", get(handlers::get_records_handler))
        .route("/salary", get(handlers::get_particular_salary))
        .route("/sales-plan", put(handlers::update_plan))
        .with_state(state)
        .fallback_service(ServeDir::new("dist"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
