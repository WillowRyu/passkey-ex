use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::{database::credentials, utils::app_error::AppError};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    cred_id: String,
    new_name: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    data: credentials::Model,
}

pub async fn handle_update_cred_name(
    Extension(db): Extension<DatabaseConnection>,
    payload: Json<Payload>,
) -> Result<Json<ResponseData>, AppError> {
    let new_cred = credentials::ActiveModel {
        id: Set(payload.cred_id.to_owned()),
        name: Set(payload.new_name.to_owned()),
        ..Default::default()
    };

    let cred = credentials::Entity::update(new_cred).exec(&db).await?;

    Ok(Json(ResponseData { data: cred }))
}
