use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::utils::app_error::AppError;

pub async fn middle_ware_csrf(request: Request, next: Next) -> Result<Response, Response> {
    let header = request.headers().get("X-Requested-With");
    if header.is_none() || header.unwrap() != "XMLHttpRequest" {
        return Err(
            AppError::new("Missing CSRF token".to_owned(), StatusCode::BAD_REQUEST).into_response(),
        );
    }

    Ok(next.run(request).await)
}
