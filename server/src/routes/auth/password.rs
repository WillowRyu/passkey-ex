use axum::{http::StatusCode, response::Response, Extension, Json};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct RequestPassword {
    password: String,
}

pub async fn password(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Json(payload): Json<RequestPassword>,
) -> Result<StatusCode, Response> {
    dbg!(&payload.password);

    let session_user = session.get_value("username").await.unwrap();
    dbg!(&session_user);

    Ok(StatusCode::OK)
}
