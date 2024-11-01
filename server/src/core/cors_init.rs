use std::str::FromStr;

use axum::http::{header::CONTENT_TYPE, HeaderName, Method};
use tower_http::cors::CorsLayer;

pub fn cors_init() -> CorsLayer {
    let allow_origins = ["http://localhost:5173".parse().unwrap()];

    CorsLayer::new()
        .allow_origin(allow_origins)
        .allow_credentials(true)
        .allow_headers([
            CONTENT_TYPE,
            HeaderName::from_str("x-requested-with").unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
}
