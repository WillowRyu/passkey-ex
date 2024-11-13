use axum::http::StatusCode;

use super::app_error::AppError;

pub fn user_not_found_error() -> AppError {
    AppError::new(
        "user not found".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn key_not_found_error() -> AppError {
    AppError::new(
        "key not found".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub fn credential_not_found_error() -> AppError {
    AppError::new(
        "credential not found".to_owned(),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}
