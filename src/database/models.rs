use chrono::{DateTime, Utc};
use serde::Serialize;

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
