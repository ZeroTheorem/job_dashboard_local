use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tower::util::ServiceExt;
mod common;
use common::setup;

#[sqlx::test(migrator = "job_dashboard_backend::database::MIGRATOR")]
async fn create_record_test(pool: PgPool) {
    let (app, pool) = setup(pool);

    let request = Request::builder()
        .method("POST")
        .uri("/records")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "employee": "Даша",
                "revenue": 50000
            })
            .to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let count = sqlx::query!("SELECT COUNT(*) FROM daily_revenue")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.count.unwrap(), 1);
}
