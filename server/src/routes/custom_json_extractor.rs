use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json, RequestExt,
};
use validator::Validate;

use super::validate_with_serde::RequestUser;

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>, _>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        if let Err(errors) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
        }

        Ok(user)
    }
}
