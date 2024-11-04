use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::{database::users, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct UpdateUsername {
    display_name: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    data: users::Model,
}

pub async fn handle_update_username(
    Extension(db): Extension<DatabaseConnection>,
    Extension(ex_user): Extension<users::Model>,
    Json(payload): Json<UpdateUsername>,
) -> Result<Json<ResponseUser>, Response> {
    if payload.display_name.is_empty() {
        return Err(
            AppError::new("username is empty".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    if ex_user.id.is_empty() {
        return Err(
            AppError::new("User not found".to_owned(), StatusCode::NOT_FOUND).into_response(),
        );
    }

    let new_user = users::ActiveModel {
        displayname: Set(payload.display_name.to_owned()),
        id: Set(ex_user.id.to_owned()),
        ..Default::default()
    };

    if let Ok(user) = users::Entity::update(new_user).exec(&db).await {
        Ok(Json(ResponseUser { data: user }))
    } else {
        Err(AppError::new(
            "Failed to update user".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response())
    }
}
