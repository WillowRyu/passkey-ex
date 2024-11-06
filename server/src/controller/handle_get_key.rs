use axum::{
    extract::Request,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::{
    database::{credentials, helpers_credentials::get_keys_by_user_id, users},
    utils::helpers_app_error::user_not_found_error,
};

#[derive(Serialize)]
pub struct ResponseKey {
    data: Vec<credentials::Model>,
}

pub async fn handle_get_key(
    Extension(db): Extension<DatabaseConnection>,
    request: Request,
) -> Result<Json<ResponseKey>, Response> {
    if let Some(user) = request.extensions().get::<users::Model>() {
        let keys = get_keys_by_user_id(&db, &user.id).await;

        match keys {
            Ok(keys) => Ok(Json(ResponseKey { data: keys })),
            Err(err) => Err(err.into_response()),
        }
    } else {
        Err(user_not_found_error().into_response())
    }
}
