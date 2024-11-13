use axum::{Extension, Json};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::{json, Value};
use tower_sessions::Session;

use crate::{
    database::{
        helpers_credentials::find_credentials_by_id, helpers_users::find_user_by_id, users,
    },
    model::const_value,
    utils::{app_error::AppError, base64_util::base64_url_encode},
};

#[derive(Serialize)]
struct Authenticator {
    credential_public_key: String,
    credential_id: String,
    transports: Vec<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    data: users::Model,
}

pub async fn handle_signin_response(
    Extension(db): Extension<DatabaseConnection>,
    session: Session,
    Json(payload): Json<Value>,
) -> Result<Json<ResponseData>, AppError> {
    let expected_challenge = session
        .get::<String>(const_value::CHALLENGE_KEY)
        .await
        .unwrap();

    // example host
    let expected_origin = "http://localhost:5173";
    let expected_rp_id = "localhost";

    let cred = find_credentials_by_id(&db, payload["id"].as_str().unwrap()).await?;

    dbg!(&cred);
    let user = find_user_by_id(&db, &cred.user_id.unwrap()).await?;

    let authenticator: Authenticator = Authenticator {
        credential_public_key: base64_url_encode(cred.publickey),
        credential_id: base64_url_encode(cred.id),
        transports: cred.transports,
    };

    let request_post = &json!({
      "response": payload,
      "expectedChallenge": expected_challenge,
      "expectedOrigin": expected_origin,
      "expectedRPID": expected_rp_id,
      "authenticator": authenticator
    });

    reqwest::Client::new()
        .post("http://localhost:3001/erify-auth-credentials")
        .json(&request_post)
        .send()
        .await?;

    session.remove::<String>(const_value::CHALLENGE_KEY).await?;
    session.insert(const_value::SIGNED_IN_KEY, "yes").await?;

    Ok(Json(ResponseData { data: user }))
}
