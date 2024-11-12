use axum::{extract::Request, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;

use crate::{
    database::{helpers_credentials::get_keys_by_user_id, users},
    model::const_value,
    utils::{
        app_error::AppError, base64_util::base64_url_decode,
        helpers_app_error::user_not_found_error,
    },
};

#[derive(Serialize, Deserialize, Debug)]
struct ExcludeCredentials {
    id: String,
    r#type: String,
    transport: Vec<String>,
}

#[derive(Serialize)]
pub struct RespnseValue {
    data: Value,
}

pub async fn handle_register_request(
    Extension(db): Extension<sea_orm::DatabaseConnection>,
    session: Session,
    request: Request,
) -> Result<Json<RespnseValue>, AppError> {
    if let Some(user) = request.extensions().get::<users::Model>() {
        let mut exclude_credentials: Vec<ExcludeCredentials> = vec![];
        if let Ok(credentials) = get_keys_by_user_id(&db, &user.id).await {
            credentials.iter().for_each(|cred| {
                let id_buffer = base64_url_decode(&cred.id).unwrap_or_else(|_| vec![]);
                exclude_credentials.push(ExcludeCredentials {
                    id: String::from_utf8(id_buffer).unwrap_or_else(|_| "".to_string()),
                    r#type: "public-key".to_string(),
                    transport: cred.transports.clone(),
                });
            });
        }

        let user_id = user.id.as_bytes().to_vec();

        let request_props = &json!({
            "rpName": "SimpleWebAuthn Example",
            "rpID": "localhost",
            "userID": &user_id,
            "userName": &user.username,
            "userDisplayName": &user.displayname,
            "attestationType": "none",
            "excludeCredentials": exclude_credentials,
            "authenticatorSelection": {
                "authenticatorAttachment": "platform",
                "requireResidentKey": true
            },
        });

        let resp = reqwest::Client::new()
            .post("http://localhost:3001/generate-options")
            .json(&request_props)
            .send()
            .await?;

        let json_resp = resp.json::<Value>().await?;

        session
            .insert(const_value::CHALLENGE_KEY, &json_resp.get("challenge"))
            .await?;

        Ok(Json(RespnseValue { data: json_resp }))
    } else {
        Err(user_not_found_error())
    }
}
