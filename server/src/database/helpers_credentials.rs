use crate::utils::{app_error::AppError, helpers_app_error::key_not_found_error};
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
        .map_err(|_| {
            dbg!("error when find key by user id");
            key_not_found_error()
        })?;

    if keys.is_empty() {
        Ok(vec![])
    } else {
        Ok(keys)
    }
}
