use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRecordBody {
    pub employee: String,
    pub revenue: i64,
}

#[derive(Deserialize)]
pub struct UpdatePlanBody {
    pub plan: i64,
}
