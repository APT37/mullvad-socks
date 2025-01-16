use crate::proxydata::ProxyData;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json, Response},
};
use mvs_lib::{Filter, Offline, Style, DEFAULT_WEIGHT};
use serde::Deserialize;
use tokio::sync::watch::Receiver;

#[derive(Debug, Deserialize)]
pub struct Params {
    cities: Option<String>,
    countries: Option<String>,
    datacenters: Option<String>,
    weight: Option<u16>,
    offline: Option<Offline>,
    style: Option<Style>,
    scheme: Option<bool>,
    port: Option<bool>,
}

#[axum_macros::debug_handler]
pub async fn filter(
    State(data): State<Receiver<ProxyData>>,
    Query(params): Query<Params>,
) -> Response {
    let proxies = data.borrow().proxies.clone();

    let mut filter = Filter::new();

    let splitter = |loc: String| loc.split(',').map(String::from).collect::<Vec<String>>();

    if let Some(countries) = params.countries.map(splitter) {
        if !countries.is_empty() {
            filter.set_countries(&countries);
        }
    }

    if let Some(cities) = params.cities.map(splitter) {
        if !cities.is_empty() {
            filter.set_cities(&cities);
        }
    }

    if let Some(datacenters) = params.datacenters.map(splitter) {
        if !datacenters.is_empty() {
            filter.set_cities(&datacenters);
        }
    }

    filter
        .set_weight(params.weight.unwrap_or(DEFAULT_WEIGHT))
        .set_offline(params.offline.unwrap_or_default())
        .set_style(params.style.unwrap_or_default())
        .set_scheme(params.scheme.unwrap_or_default())
        .set_port(params.port.unwrap_or_default());

    Json(filter.apply(proxies)).into_response()
}
