use crate::utils::{app_error::AppError, helpers_app_error::user_not_found_error};
use sea_orm::*;

use super::users;

pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<users::Model, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|_| {
            dbg!("error when find user by username");
            user_not_found_error()
        })?;

    user.ok_or_else(user_not_found_error)
}

pub async fn find_user_by_id(db: &DatabaseConnection, id: &str) -> Result<users::Model, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(db)
        .await
        .map_err(|_| {
            dbg!("error when find user by Id");
            user_not_found_error()
        })?;

    user.ok_or_else(user_not_found_error)
}
