use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};

use super::middle_ware_custom_header::HeaderMessage;

pub async fn set_middle_ware_custom_header(
  headers: HeaderMap,
  mut request: Request,
  next: Next
) -> Result<Response, StatusCode>{
  match get_header_message(&headers) {
    Some(message) => {
      request.extensions_mut().insert(HeaderMessage(message.to_owned()));
      Ok(next.run(request).await)
    }
    _ => Err(StatusCode::BAD_REQUEST)?
  }
}

fn get_header_message(headers: &HeaderMap) -> Option<&str> {
  headers.get("message").and_then(|value| value.to_str().ok()).or(None)
}