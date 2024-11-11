use std::collections::HashMap;

use axum::{http::HeaderMap, Extension, Json};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;

use crate::{
    database::{credentials, users},
    utils::{
        app_error::AppError,
        base64_util::{base64_url_decode, base64_url_encode, base64_url_encode_json},
        header_parse::user_agent_handler,
    },
};

#[derive(Serialize)]
pub struct ResponseUser {
    data: users::Model,
}

#[derive(Deserialize, Serialize)]
struct RegistrationInfo {
    credential_id: String,
    credential_public_key: HashMap<String, u8>, // 각 키-값이 String -> u8
}

#[derive(Deserialize, Serialize)]
struct RequestData {
    registration_info: RegistrationInfo,
    verified: bool,
}

impl RegistrationInfo {
    // `credentialPublicKey`를 Vec<u8>으로 변환하는 함수
    fn get_public_key_as_vec(&self) -> Vec<u8> {
        let mut public_key: Vec<u8> = self
            .credential_public_key
            .iter()
            .map(|(_, &value)| value)
            .collect();
        public_key
    }
}

fn convert_public_key_to_vec(public_key: &Value) -> Vec<u8> {
    if let Some(obj) = public_key.as_object() {
        obj.iter()
            .filter_map(|(_, v)| v.as_u64()) // u64로 변환 가능한 값만 필터링
            .map(|v| v as u8) // u64 -> u8로 변환
            .collect()
    } else {
        Vec::new() // Object가 아니면 빈 벡터 반환
    }
}

pub async fn handle_register_response(
    Extension(_db): Extension<DatabaseConnection>,
    Extension(_user): Extension<users::Model>,
    session: Session,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode, AppError> {
    let expected_challenge = session.get::<String>("challenge").await.unwrap();

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

    dbg!("resp {:?}", &resp);

    let json_resp = resp.json::<Value>().await?;

    if let Some(credential_info) = json_resp.get("registrationInfo") {
        if let Some(public_key) = credential_info.get("credentialPublicKey") {
            let veckey = convert_public_key_to_vec(public_key);
            let public_key_vec = base64_url_encode(&veckey);
            dbg!("converted public_key: {:?}", public_key_vec);
        }
    }

    // println!("{:?}", public_key_vec);

    // let registration_info = json_resp.get("registrationInfo").ok_or_else(|| {
    //     AppError::new(
    //         "registrationInfo is missing".to_string(),
    //         StatusCode::BAD_REQUEST,
    //     )
    // })?;

    // dbg!("registration_info {:?}", &registration_info);

    // let base64_credential_id = base64_url_encode(&registration_info.credentialId);

    // dbg!("base64_credential_id {}", &base64_credential_id);

    // let base64_credntial_public_key = &registration_info.credential_public_key;

    // dbg!(
    //     "base64_credntial_public_key {}",
    //     &base64_credntial_public_key
    // );

    // let user_agent = user_agent_handler(&headers).await;
    // dbg!("user_agent {}", &user_agent);

    // let transports = set_or_error(
    //     payload.get("response").and_then(|v| {
    //         v.get("transports").and_then(|v| v.as_array()).map(|arr| {
    //             arr.iter()
    //                 .filter_map(|item| item.as_str().map(String::from))
    //                 .collect()
    //         })
    //     }),
    //     "transports",
    // )?;
    // dbg!("transports {:?}", &transports);

    // let new_credential = credentials::ActiveModel {
    //     id: Set(base64_credential_id),
    //     publickey: Set(base64_credntial_public_key),
    //     user_id: Set(Some(_user.id)),
    //     transports: Set(transports),
    //     name: Set(user_agent),
    // };

    // dbg!("new_credential {}", &new_credential);

    Ok(StatusCode::OK)
}
