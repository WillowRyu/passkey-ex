use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_sessions::Session;

use crate::{model::const_value, utils::app_error::AppError};

#[derive(Serialize, Deserialize, Debug)]
struct ExcludeCredentials {
    id: String,
    r#type: String,
    transport: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AuthGenerateOptions {
    challenge: String,
    rp_id: String,
    timeout: u32,
    user_verification: String,
    allow_credentials: Vec<ExcludeCredentials>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Options {
    options: AuthGenerateOptions,
}

#[derive(Serialize)]
pub struct ResponseData {
    data: AuthGenerateOptions,
}

pub async fn handle_signin_request(session: Session) -> Result<Json<ResponseData>, AppError> {
    let request_props = &json!({
        "rpID": "localhost",
        "allowCredentials": [],
    });

    let resp = reqwest::Client::new()
        .post("http://localhost:3001/generate-auth-options")
        .json(&request_props)
        .send()
        .await?;

    let json_resp = resp.json::<Options>().await?;

    session
        .insert(const_value::CHALLENGE_KEY, &json_resp.options.challenge)
        .await?;

    Ok(Json(ResponseData {
        data: json_resp.options,
    }))
}
