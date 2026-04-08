use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

#[derive(Serialize)]
pub struct Record {
    pub id: i64,
    pub employee: String,
    pub revenue: i64,
    pub date: DateTime<Utc>,
}

pub struct RevenueStats {
    pub total_shifts: Option<i64>,
    pub total_revenue: Option<i64>,
    pub average: Option<i64>,
}

impl Database {
    pub async fn builder() -> anyhow::Result<Self> {
        let database_url = &std::env::var("DATABASE_URL").context("DATABASE_URL not found")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("error while migrate")?;
        Ok(Database { pool })
    }

    pub async fn create_record(&self, worker_name: String, revenue: i64) -> anyhow::Result<i64> {
        let created_record_id = sqlx::query!(
            "INSERT INTO daily_revenue (worker_name, revenue) VALUES ($1, $2) RETURNING id",
            worker_name,
            revenue
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(created_record_id.id)
    }

    pub async fn delete_last_record(&self) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM daily_revenue WHERE id = (SELECT MAX(id) FROM daily_revenue)",)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn create_plan(&self, new_value: i64) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO plan (value) VALUES ($1)", new_value)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_worker_monthly_revenue_stats(
        &self,
        worker_name: &str,
        particular_date: &utc::DateBounds,
    ) -> anyhow::Result<RevenueStats> {
        let revenue = sqlx::query_as!(
            RevenueStats,
            "SELECT COUNT(*) as total_shifts,
                     SUM(revenue)::BIGINT as total_revenue,
                     AVG(revenue)::BIGINT as average
                     FROM daily_revenue
                     WHERE created_at >= $1
                     AND created_at < $2
                     AND worker_name = $3",
            particular_date.start,
            particular_date.end,
            worker_name
        )
        .fetch_one(&self.pool)
        .await?;
        return Ok(revenue);
    }
    pub async fn get_records(&self, date_bounds: utc::DateBounds) -> anyhow::Result<Vec<Record>> {
        let records = sqlx::query_as!(
            Record,
            "SELECT id, worker_name as employee, revenue, created_at as date
            FROM daily_revenue
            WHERE created_at >= $1
            AND created_at < $2",
            date_bounds.start,
            date_bounds.end
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(records)
    }
    pub async fn get_plan(&self, date_bounds: &utc::DateBounds) -> anyhow::Result<i64> {
        let plan = sqlx::query!(
            "SELECT value
            FROM plan
            WHERE created_at >= $1
            AND created_at < $2
            ORDER BY created_at DESC
            LIMIT 1",
            date_bounds.start,
            date_bounds.end
        )
        .fetch_optional(&self.pool)
        .await?;
        match plan {
            Some(plan) => Ok(plan.value),
            None => Ok(1_000_000),
        }
    }
}
