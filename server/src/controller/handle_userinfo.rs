use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use crate::{database::users, utils::app_error::AppError};

#[derive(Serialize)]
pub struct ResponseUser {
    data: users::Model,
}

pub async fn handle_userinfo(request: Request) -> Result<Json<ResponseUser>, Response> {
    if let Some(user) = request.extensions().get::<users::Model>() {
        Ok(Json(ResponseUser { data: user.clone() }))
    } else {
        Err(AppError::new("User not found".to_owned(), StatusCode::NOT_FOUND).into_response())
    }
}
