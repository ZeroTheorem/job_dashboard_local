use axum::body::Body;
use axum::http::{Request, StatusCode};
use job_dashboard_backend::database;
use sqlx::PgPool;
use tower::util::ServiceExt;
mod common;
use common::setup;

#[sqlx::test(migrator = "job_dashboard_backend::database::MIGRATOR")]
async fn delete_record_test(pool: PgPool) {
    let (app, pool) = setup(pool);
    let database = database::Database { pool: pool.clone() };

    database
        .create_record("Алена".to_string(), 50000)
        .await
        .unwrap();

    let request = Request::builder()
        .method("DELETE")
        .uri("/records/1")
        .header("content-type", "application/json")
        .body(Body::from(""))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let count = sqlx::query!("SELECT COUNT(*) FROM daily_revenue")
        .fetch_one(&database.pool)
        .await
        .unwrap();
    assert_eq!(count.count.unwrap(), 0);
}
