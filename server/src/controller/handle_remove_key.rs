use axum::{extract::Query, Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::{database::credentials, utils::app_error::AppError};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RemoveKeyRequest {
    cred_id: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    status: String,
}

pub async fn handle_remove_key(
    Extension(db): Extension<DatabaseConnection>,
    query: Query<RemoveKeyRequest>,
) -> Result<Json<ResponseData>, AppError> {
    dbg!(&query.cred_id);
    credentials::Entity::delete_by_id(&query.cred_id)
        .exec(&db)
        .await?;

    Ok(Json(ResponseData {
        status: "OK".to_owned(),
    }))
}
