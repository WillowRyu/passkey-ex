use std::error::Error;

use base64::Engine;
use rand::Rng;
use serde_json::Value;

pub fn base64_url_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn base64_url_encode_json(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(|s| base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(s.as_bytes()))
}

pub fn base64_url_decode(encoded: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(decoded)
}

pub fn generate_base64_id() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
}
