use axum::response::{ IntoResponse, Json, Response };
use const_format::formatcp;

#[derive(serde::Serialize, Clone, Copy)]
struct Version {
    version: &'static str,
}

impl Version {
    const fn new(version: &'static str) -> Self {
        Self { version }
    }
}

const VERSION: Version = Version::new(
    formatcp!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
);

#[axum_macros::debug_handler]
pub async fn version() -> Response {
    Json(VERSION).into_response()
}
