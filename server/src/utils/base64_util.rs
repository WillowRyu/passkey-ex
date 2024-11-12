use std::error::Error;

use base64::Engine;
use rand::Rng;

pub fn base64_url_encode<T>(data: T) -> String
where
    T: AsRef<[u8]>,
{
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn base64_url_decode<T>(encoded: T) -> Result<Vec<u8>, Box<dyn Error>>
where
    T: AsRef<[u8]>,
{
    let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(decoded)
}

pub fn generate_base64_id() -> String {
    let random_bytes: [u8; 32] = rand::thread_rng().gen();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
}
