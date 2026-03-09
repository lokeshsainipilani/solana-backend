use axum::{http::StatusCode, Json, response::{IntoResponse, Response}};
use crate::models::ErrorResponse;

pub fn error_response(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(ErrorResponse {
            success: false,
            error: message.to_string(),
        }),
    ).into_response()
}