use axum::http::HeaderMap;
use regex::Regex;

pub async fn user_agent_handler(headers: &HeaderMap) -> String {
    let user_agent = headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    let platform_re = Regex::new(r"\(([^;]+);?").unwrap();

    let platform = platform_re
        .captures(user_agent)
        .and_then(|caps| caps.get(1).map(|m| m.as_str()));

    match platform {
        Some(platform) => platform.to_string(),
        None => "Platform not found in user-agent".to_string(),
    }
}
