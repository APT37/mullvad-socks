use axum::response::{IntoResponse, Json, Response};

const VERSION: &str =
    const_format::formatcp!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

#[derive(serde::Serialize)]
struct Version {
    version: &'static str,
}

#[axum_macros::debug_handler]
pub async fn version() -> Response {
    Json(Version { version: VERSION }).into_response()
}
