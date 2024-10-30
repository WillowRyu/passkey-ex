use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
  message: String,
  count: i32,
  username: String
}

pub async fn get_json() -> Json<Data> {
  let data = Data {
    message: "Hello from get_json!".to_string(),
    count: 100,
    username: "user123".to_string()
  };

  Json(data)
}