use crate::utils::{
    app_error::AppError,
    helpers_app_error::{credential_not_found_error, key_not_found_error},
};
use sea_orm::*;

use super::credentials;

pub async fn get_keys_by_user_id(
    db: &DatabaseConnection,
    user_id: &str,
) -> Result<Vec<credentials::Model>, AppError> {
    let keys = credentials::Entity::find()
        .filter(credentials::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|_| key_not_found_error())?;

    if keys.is_empty() {
        Ok(vec![])
    } else {
        Ok(keys)
    }
}

pub async fn find_credentials_by_id(
    db: &DatabaseConnection,
    id: &str,
) -> Result<credentials::Model, AppError> {
    let user = credentials::Entity::find()
        .filter(credentials::Column::Id.eq(id))
        .one(db)
        .await
        .map_err(|_| credential_not_found_error())?;

    user.ok_or_else(credential_not_found_error)
}
