use std::error::Error;

use base64::Engine;
use rand::RngCore;

pub fn base64_url_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn base64_url_decode(encoded: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(decoded)
}

pub fn generate_base64_id() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}
