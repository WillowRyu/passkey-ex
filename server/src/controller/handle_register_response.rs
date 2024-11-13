use std::collections::HashMap;

use axum::{
    http::{HeaderMap, StatusCode},
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;

use crate::{
    database::{credentials, users},
    model::const_value,
    utils::{
        app_error::AppError, base64_util::base64_url_encode, header_parse::user_agent_handler,
    },
};

#[derive(Serialize)]
pub struct ResponseUser {
    data: users::Model,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RegistrationInfo {
    credential_id: String,

    #[serde(deserialize_with = "deserialize_public_key")]
    credential_public_key: Vec<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RequestData {
    registration_info: RegistrationInfo,
    verified: bool,
}

fn deserialize_public_key<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    // `HashMap<String, u8>`로 데이터를 받음
    let map: HashMap<String, u8> = HashMap::deserialize(deserializer)?;

    // 키를 숫자로 변환하여 정렬하고, 해당 값을 Vec<u8>에 추가
    let mut values: Vec<(usize, u8)> = map
        .into_iter()
        .filter_map(|(k, v)| k.parse::<usize>().ok().map(|i| (i, v)))
        .collect();

    values.sort_by_key(|&(k, _)| k);

    Ok(values.into_iter().map(|(_, v)| v).collect())
}

pub async fn handle_register_response(
    Extension(db): Extension<DatabaseConnection>,
    Extension(user): Extension<users::Model>,
    session: Session,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<Json<ResponseUser>, AppError> {
    let expected_challenge = session
        .get::<String>(const_value::CHALLENGE_KEY)
        .await
        .unwrap();

    // example host
    let expected_origin = "http://localhost:5173";
    let expected_rp_id = "localhost";

    let request_post = &json!({
      "response": payload,
      "expectedChallenge": &expected_challenge,
      "expectedOrigin": &expected_origin,
      "expectedRPID": &expected_rp_id,
      "requireUserVerification": false,
    });

    let resp = reqwest::Client::new()
        .post("http://localhost:3001/verify-credentials")
        .json(&request_post)
        .send()
        .await?;

    let json_resp = resp.json::<RequestData>().await?;

    if !json_resp.verified {
        return Err(AppError::new(
            "Verification failed".to_string(),
            StatusCode::UNAUTHORIZED,
        ));
    }

    dbg!(&json_resp);

    // let base64_credential_id = base64_url_encode(json_resp.registration_info.credential_id);
    let base64_credntial_public_key =
        base64_url_encode(&json_resp.registration_info.credential_public_key);
    let user_agent = user_agent_handler(&headers).await;
    let transports = payload
        .get("response")
        .and_then(|response| response.get("transports"))
        .and_then(|transports| transports.as_array())
        .ok_or(AppError::new(
            "transports error".to_string(),
            StatusCode::BAD_REQUEST,
        ))?;

    let transports_vec = transports
        .iter()
        .filter_map(|t| t.as_str().map(|s| s.to_string()))
        .collect();

    let new_credential = credentials::ActiveModel {
        id: Set(json_resp.registration_info.credential_id),
        publickey: Set(base64_credntial_public_key),
        user_id: Set(Some(user.id.clone())),
        transports: Set(transports_vec),
        name: Set(user_agent),
    };

    new_credential.insert(&db).await.map_err(|_| {
        AppError::new(
            "Failed to save credential".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    session.remove::<String>(const_value::CHALLENGE_KEY).await?;
    session.insert(const_value::SIGNED_IN_KEY, "yes").await?;

    Ok(Json(ResponseUser { data: user }))
}
