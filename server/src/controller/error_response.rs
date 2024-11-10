use crate::utils::app_error::AppError;
use axum::{body::Body, http::StatusCode, response::Response};
use serde_json::json;

impl From<serde_json::Error> for AppError {
    fn from(_: serde_json::Error) -> Self {
        AppError::new(
            "JSON serialization error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: reqwest::Error) -> Self {
        AppError::new("HTTP error".to_string(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<tower_sessions::session::Error> for AppError {
    fn from(_: tower_sessions::session::Error) -> Self {
        AppError::new(
            "Session error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<AppError> for Response<Body> {
    fn from(error: AppError) -> Self {
        let body = Body::from(json!({ "error": error.message }).to_string());
        Response::builder().status(error.code).body(body).unwrap()
    }
}
