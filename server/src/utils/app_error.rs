use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub struct AppError {
    message: String,
    code: StatusCode,
}

impl AppError {
    pub fn new(message: String, code: StatusCode) -> Self {
        Self { message, code }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            message: self.message,
            code: self.code.to_string(),
        });

        (self.code, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
}
