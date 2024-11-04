use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use tower_sessions::Session;

use crate::{
    database::helpers_users::find_user_by_username, model::const_value, utils::app_error::AppError,
};

/**
 * todo session 까지 완료
 * 이제 users 에서 가져와야 함.
 */

pub async fn middle_ware_session(
    Extension(db): Extension<sea_orm::DatabaseConnection>,
    session: Session,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    let username = session
        .get::<String>(const_value::USERNAME_KEY)
        .await
        .unwrap();

    if session
        .get_value(const_value::SIGNED_IN_KEY)
        .await
        .unwrap()
        .is_none()
        || username.is_none()
    {
        return Err(
            AppError::new("not signed in".to_owned(), StatusCode::UNAUTHORIZED).into_response(),
        );
    }

    match find_user_by_username(&db, &username.unwrap()).await {
        Ok(user) => {
            dbg!("session user {}", &user);
            request.extensions_mut().insert(user);
        }
        Err(err) => return Err(err.into_response()),
    }

    Ok(next.run(request).await)
}
