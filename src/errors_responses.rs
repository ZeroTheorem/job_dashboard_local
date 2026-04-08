use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub enum Error {
    BadRequest,
    Internal,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let (status_code, json) = match self {
            Error::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "BadRequest"})),
            ),
            Error::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        };
        (status_code, json).into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Internal
    }
}
