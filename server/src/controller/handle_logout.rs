use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;
use tower_sessions::Session;

use crate::utils::app_error::AppError;

#[derive(Serialize)]
pub struct ResponseData {
    status: String,
}

pub async fn handle_logout(session: Session) -> Result<Json<ResponseData>, AppError> {
    match session.delete().await {
        Ok(_) => Ok(Json(ResponseData {
            status: StatusCode::OK.to_string(),
        })),
        Err(_) => Err(AppError::new(
            "Failed to logout".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
