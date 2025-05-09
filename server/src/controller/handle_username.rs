use crate::{
    database::{helpers_users::find_user_by_username, users},
    utils::{app_error::AppError, base64_util::generate_base64_id},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};

use regex::Regex;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use super::helpers::store_username_in_session;

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    data: users::Model,
}

pub async fn handle_username(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Json(payload): Json<CreateUser>,
) -> Result<Json<ResponseUser>, Response> {
    let username_regex = Regex::new(r"^[a-zA-Z0-9@\.\-_]+$").unwrap();

    if !username_regex.is_match(&payload.username) {
        return Err(
            AppError::new("Invalid username".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    let user = match find_user_by_username(&db, &payload.username).await {
        Ok(user) => user,
        Err(_) => {
            let id = generate_base64_id();
            let new_user = users::ActiveModel {
                username: Set(payload.username.to_owned()),
                displayname: Set(payload.username.to_owned()),
                id: Set(id),
            };

            new_user.insert(&db).await.map_err(|_| {
                AppError::new(
                    "Failed to save user".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response()
            })?
        }
    };

    store_username_in_session(&session, payload.username.to_owned()).await?;

    Ok(Json(ResponseUser { data: user }))
}
