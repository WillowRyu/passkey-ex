use axum::Json;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email"))]
    pub username: Option<String>,
    #[validate(length(min = 8, message = "must be at least 8 characters"))]
    pub password: String
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}