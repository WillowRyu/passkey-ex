use std::collections::HashMap;

use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sea_orm::*;
use serde::Serialize;

use crate::{
    database::{credentials, helpers_credentials::get_keys_by_user_id, users},
    utils::{base64_util::base64_url_decode, helpers_app_error::user_not_found_error},
};

#[derive(Serialize)]
struct ExcludeCredentials {
    id: String,
    r#type: String,
    transport: Vec<String>,
}

#[derive(Serialize)]
struct AuthenticatorSelection {
    authenticatorAttachment: String,
    requireResidentKey: bool,
}

pub async fn handle_register_request(
    Extension(db): Extension<DatabaseConnection>,
    request: Request,
) -> Result<StatusCode, Response> {
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

        let authenticator_selection = AuthenticatorSelection {
            authenticatorAttachment: "platform".to_string(),
            requireResidentKey: true,
        };

        let attestationType = "none";

        let challange = reqwest::get("http://localhost:3001/generate-options").await;

        match reqwest::get("http://localhost:3001/generate-options").await {
            Ok(resp) => {
                let json: Result<serde_json::Value, reqwest::Error> = resp.json().await;
                if json.is_ok() {
                    println!("{:#?}", json);
                    return Ok(StatusCode::OK);
                } else {
                    return Err(Response::new("error".to_string()).into_response());
                }
            }
            Err(_) => {
                return Err(Response::new("error".to_string()).into_response());
            }
        }
    }

    Ok(StatusCode::OK)
}
