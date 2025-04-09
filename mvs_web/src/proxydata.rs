use mvs_lib::{ Proxy, API_URL };
use reqwest::blocking::Client;

#[derive(Debug, Clone, Default)]
pub struct ProxyData {
    pub proxies: Vec<Proxy>,
    pub locations: Locations,
}

#[derive(Debug, Clone, Default)]
pub struct Locations {
    pub countries: Vec<String>,
    pub cities: Vec<String>,
    pub datacenters: Vec<String>,
}

impl Locations {
    fn new(proxies: &[Proxy]) -> Self {
        Self {
            countries: Proxy::countries(proxies),
            cities: Proxy::cities(proxies),
            datacenters: Proxy::datacenters(proxies),
        }
    }
}

impl ProxyData {
    pub fn new() -> Result<Self, reqwest::Error> {
        // fetch latest proxy list
        let proxies = Client::new().get(API_URL).send()?.json::<Vec<Proxy>>()?;

        // extract locations from proxy list
        let locations = Locations::new(&proxies);

        // construct output object
        Ok(ProxyData { proxies, locations })
    }
}
