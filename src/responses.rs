use axum::{
    Json,
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::database::models::Record;

pub enum SuccessResponse {
    CreateRecord {
        record_id: i64,
    },
    GetDashBoard {
        shifts_this_month: i64,
        revenue: i64,
        sales_plan: i64,
        salary: i64,
        potential_salary: i64,
        avg_employee_1: i64,
        avg_employee_2: i64,
    },
    DeleteRecord,
    GetRecords {
        records: Vec<Record>,
    },
    WorkerSalary {
        worker_salary: i64,
    },
    CreatePlan {
        plan: i64,
    },
}

impl IntoResponse for SuccessResponse {
    fn into_response(self) -> Response<Body> {
        let (status_code, json) = match self {
            Self::DeleteRecord => (StatusCode::OK, Json(json!({"success": true}))),
            Self::GetRecords { records } => (StatusCode::OK, Json(json!({"records": records}))),
            Self::CreatePlan { plan } => {
                (StatusCode::OK, Json(json!({"success": true, "plan": plan})))
            }
            Self::CreateRecord { record_id } => (
                StatusCode::CREATED,
                Json(json!({"success": true, "id": record_id})),
            ),
            Self::GetDashBoard {
                shifts_this_month,
                revenue,
                salary,
                potential_salary,
                sales_plan,
                avg_employee_1,
                avg_employee_2,
            } => (
                StatusCode::OK,
                Json(json!({
                    "shiftsThisMonth":  shifts_this_month,
                    "revenue":          revenue,
                    "salesPlan":        sales_plan,
                    "salary":           salary,
                    "potentialSalary":  potential_salary,
                    "avgEmployee1":     avg_employee_1,
                    "avgEmployee2":     avg_employee_2
                })),
            ),
            Self::WorkerSalary { worker_salary } => (
                StatusCode::OK,
                Json(json!({
                    "salary": worker_salary
                })),
            ),
        };
        (status_code, json).into_response()
    }
}

pub enum ErrorResponse {
    BadRequest,
    Internal,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response<Body> {
        let (status_code, json) = match self {
            ErrorResponse::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "BadRequest"})),
            ),
            ErrorResponse::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        };
        (status_code, json).into_response()
    }
}

impl From<anyhow::Error> for ErrorResponse {
    fn from(_: anyhow::Error) -> Self {
        Self::Internal
    }
}
