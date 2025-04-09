use crate::proxydata::ProxyData;
use axum::{
    extract::{ Query, State },
    http::StatusCode,
    response::{ IntoResponse, Json, Response },
};
use serde::Deserialize;
use tokio::sync::watch::Receiver;

#[derive(Debug, Clone, Deserialize)]
pub struct LType {
    r#type: Type,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Country,
    City,
    Datacenter,
}

// list locations
#[axum_macros::debug_handler]
pub async fn locations(
    State(data): State<Receiver<ProxyData>>,
    Query(ltype): Query<LType>
) -> Response {
    let locations = match ltype.r#type {
        Type::Country => data.borrow().locations.countries.clone(),
        Type::City => data.borrow().locations.cities.clone(),
        Type::Datacenter => data.borrow().locations.datacenters.clone(),
    };

    if locations.is_empty() {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    } else {
        Json(locations).into_response()
    }
}
