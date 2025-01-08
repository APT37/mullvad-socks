use axum::response::{IntoResponse, Json, Response};

#[derive(serde::Serialize)]
struct Version {
    version: String,
}

#[axum_macros::debug_handler]
pub async fn version() -> Response {
    Json(Version {
        version: format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
    })
    .into_response()
}
