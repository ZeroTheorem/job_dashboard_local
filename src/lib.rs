pub mod database;
mod handlers;
mod jsons;
mod responses;
mod salary;
pub mod state;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use tower_http::services::ServeDir;

pub fn create_app_with_state(state: state::AppState) -> Router {
    let app = Router::new()
        .route("/records", post(handlers::create_record_handler))
        .route("/dashboard", get(handlers::get_dashboard_handler))
        .route("/records/{id}", delete(handlers::delete_record_handler))
        .route("/records", get(handlers::get_records_handler))
        .route("/salary", get(handlers::get_particular_salary_handler))
        .route("/sales-plan", put(handlers::create_plan_handler))
        .with_state(state)
        .fallback_service(ServeDir::new("dist"));
    app
}
