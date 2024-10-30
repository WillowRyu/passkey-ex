use axum::extract::State;

pub async fn middle_ware_message(State(message): State<String>) -> String {
    message
}
