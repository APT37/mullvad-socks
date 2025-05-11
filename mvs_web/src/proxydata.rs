use mvs_lib::{ Proxy, API_URL };
use reqwest::{ blocking, Error };

#[derive(Clone, Default)]
pub struct ProxyData {
    pub proxies: Vec<Proxy>,
    pub locations: Locations,
}

#[derive(Clone, Default)]
pub struct Locations {
    pub countries: Vec<String>,
    pub cities: Vec<String>,
    // pub datacenters: Vec<String>,
}

impl Locations {
    fn new(proxies: &[Proxy]) -> Self {
        Self {
            countries: Proxy::countries(proxies),
            cities: Proxy::cities(proxies),
            // datacenters: Proxy::datacenters(proxies),
        }
    }
}

impl ProxyData {
    pub fn new() -> Result<Self, Error> {
        // fetch latest proxy list
        let proxies = blocking::get(API_URL)?.json::<Vec<Proxy>>()?;

        // extract locations from proxy list
        let locations = Locations::new(&proxies);

        // construct output object
        Ok(ProxyData { proxies, locations })
    }
}
