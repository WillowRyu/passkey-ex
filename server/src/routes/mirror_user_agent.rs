use axum::http::{header, HeaderMap};

pub async fn mirror_user_agent(headers: HeaderMap) -> String {
  headers.get(header::USER_AGENT)
    .map(|value| value.to_str().unwrap().to_string())
    .unwrap_or_else(|| "No User-Agent header found".to_string())
  
}