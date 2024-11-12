use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

use crate::{model::const_value, utils::app_error::AppError};

pub async fn store_username_in_session(
    session: &Session,
    username: String,
) -> Result<(), Response> {
    if username.is_empty() {
        return Err(
            AppError::new("username is empty".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    match session.insert(const_value::USERNAME_KEY, username).await {
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
