use dotenv::dotenv;
use job_dashboard_backend::create_app_with_state;
use job_dashboard_backend::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load env variables
    dotenv().ok();

    let state = AppState::builder().await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    let app = create_app_with_state(state);

    axum::serve(listener, app).await?;
    Ok(())
}
