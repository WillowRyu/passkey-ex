use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{
    database::{helpers_users::find_user_by_username, users},
    model::const_value,
    utils::app_error::AppError,
};

#[derive(Deserialize, Serialize)]
pub struct RequestPassword {
    password: String,
}

#[derive(Serialize)]
pub struct ResponsePassword {
    data: users::Model,
}

pub async fn handle_password(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Json(payload): Json<RequestPassword>,
) -> Result<Json<ResponsePassword>, Response> {
    if payload.password.is_empty() {
        return Err(
            AppError::new("password is empty".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    let username = session
        .get::<String>(const_value::USERNAME_KEY)
        .await
        .unwrap_or_default();

    if username.is_none() {
        return Err(
            AppError::new("Enter username first.".to_owned(), StatusCode::UNAUTHORIZED)
                .into_response(),
        );
    }

    let user = match find_user_by_username(&db, &username.unwrap()).await {
        Ok(user) => user,
        Err(err) => {
            return Err(err.into_response());
        }
    };

    match session.insert(const_value::SIGNED_IN_KEY, "yes").await {
        Ok(_) => Ok(Json(ResponsePassword { data: user })),
        _ => Err(AppError::new(
            "Failed to store singed-in in session".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()),
    }
}
