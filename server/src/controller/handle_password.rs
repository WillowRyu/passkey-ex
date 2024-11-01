use axum::{http::StatusCode, response::Response, Extension, Json};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Deserialize, Serialize)]
pub struct RequestPassword {
    password: String,
}

#[derive(Serialize)]
pub struct ResponsePassword {
    data: String,
}

const USERNAME_KEY: &str = "username";

pub async fn handle_password(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Json(payload): Json<RequestPassword>,
) -> Result<Json<ResponsePassword>, Response> {
    dbg!(&payload.password);
    dbg!(&session);
    dbg!(&session.get::<String>(USERNAME_KEY).await);

    // let session_user = session.get_value("").await.unwrap();
    // dbg!(&session_user);

    Ok(Json(ResponsePassword {
        data: "password".to_owned(),
    }))
}
