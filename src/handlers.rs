use crate::{
    errors_responses::Error,
    jsons::{CreateRecordBody, UpdatePlanBody},
    salary::{count_monthly_salary, count_old_salary},
    state::AppState,
    success_responses::SuccessResponse,
};
use axum::{extract, response::Result};
use serde::Deserialize;

const TIME_ZONE: &str = "Europe/Moscow";

#[derive(Deserialize)]
pub struct ParticularDateParams {
    year: i32,
    month: u32,
}

type AppResponse = Result<SuccessResponse, Error>;

pub async fn create_record_handler(
    extract::State(state): extract::State<AppState>,
    extract::Json(body): extract::Json<CreateRecordBody>,
) -> AppResponse {
    let worker_name = body.employee.to_lowercase();
    if worker_name != "даша" && worker_name != "алена" {
        return Err(Error::BadRequest);
    }
    let record_id = state
        .database
        .create_record(worker_name, body.revenue)
        .await?;
    Ok(SuccessResponse::CreateRecord { record_id })
}

pub async fn delete_last(extract::State(state): extract::State<AppState>) -> AppResponse {
    state.database.delete_last_record().await?;
    Ok(SuccessResponse::DeleteRecord)
}

pub async fn update_plan(
    extract::State(state): extract::State<AppState>,
    extract::Json(body): extract::Json<UpdatePlanBody>,
) -> AppResponse {
    state.database.create_plan(body.plan).await?;
    Ok(SuccessResponse::CreatePlan { plan: body.plan })
}

pub async fn get_records_handler(
    extract::State(state): extract::State<AppState>,
    date: extract::Query<ParticularDateParams>,
) -> AppResponse {
    let date_bounds = utc::period_bounds_utc(
        TIME_ZONE,
        utc::Period::ParticularMonth {
            year: date.year,
            month: date.month,
        },
    )?;
    let records = state.database.get_records(date_bounds).await?;
    Ok(SuccessResponse::GetRecords { records })
}

pub async fn get_particular_salary(
    extract::State(state): extract::State<AppState>,
    date: extract::Query<ParticularDateParams>,
) -> AppResponse {
    let date_bounds = utc::period_bounds_utc(
        TIME_ZONE,
        utc::Period::ParticularMonth {
            year: date.year,
            month: date.month,
        },
    )?;
    let worker_1_monthly_stats = state
        .database
        .get_worker_monthly_revenue_stats("алена", &date_bounds)
        .await?;

    let worker_salary = count_monthly_salary(&worker_1_monthly_stats);
    Ok(SuccessResponse::WorkerSalary { worker_salary })
}

pub async fn get_dashboard(
    extract::State(state): extract::State<AppState>,
    date: extract::Query<ParticularDateParams>,
) -> AppResponse {
    let date_bounds = utc::period_bounds_utc(
        TIME_ZONE,
        utc::Period::ParticularMonth {
            year: date.year,
            month: date.month,
        },
    )?;
    let worker_1_monthly_stats = state
        .database
        .get_worker_monthly_revenue_stats("алена", &date_bounds)
        .await?;

    let worker_2_monthly_stats = state
        .database
        .get_worker_monthly_revenue_stats("даша", &date_bounds)
        .await?;

    let sales_plan = state.database.get_plan(&date_bounds).await?;

    Ok(SuccessResponse::GetDashBoard {
        shifts_this_month: worker_1_monthly_stats.total_shifts.unwrap_or(0),
        revenue: worker_1_monthly_stats.total_revenue.unwrap_or(0)
            + worker_2_monthly_stats.total_revenue.unwrap_or(0),
        sales_plan: sales_plan,
        salary: count_monthly_salary(&worker_1_monthly_stats),
        potential_salary: count_old_salary(&worker_1_monthly_stats),
        avg_employee_1: worker_1_monthly_stats.average.unwrap_or(0),
        avg_employee_2: worker_2_monthly_stats.average.unwrap_or(0),
    })
}
