use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64::Engine;
use rand::RngCore;
use tower_sessions::Session;

use crate::utils::app_error::AppError;

pub async fn store_username_in_session(
    session: &Session,
    username: String,
) -> Result<(), Response> {
    if username.is_empty() {
        return Err(
            AppError::new("username is empty".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    match session.insert("username", username).await {
        Ok(_) => {
            dbg!("username stored in session");
            dbg!(&session);
            Ok(())
        }
        Err(_) => Err(AppError::new(
            "Failed to store username in session".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()),
    }
}
