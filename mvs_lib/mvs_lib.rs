#![allow(clippy::must_use_candidate)]
use serde::{ Deserialize, Deserializer };

pub const API_URL: &str = "https://api.mullvad.net/network/v1-beta1/socks-proxies";

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Proxy {
    online: bool,
    hostname: Option<String>,
    ipv4_address: String,
    ipv6_address: String,
    location: Location,
    weight: u16,
    port: u16,
}

impl Proxy {
    pub fn country(&self) -> &str {
        &self.location.country
    }

    pub fn city(&self) -> &str {
        &self.location.city
    }

    // pub fn datacenter(&self) -> &str {
    //     &self.location.code
    // }

    pub fn countries(proxies: &[Self]) -> Vec<String> {
        Self::locations(proxies, LType::Country)
    }

    pub fn cities(proxies: &[Self]) -> Vec<String> {
        Self::locations(proxies, LType::City)
    }

    // pub fn datacenters(proxies: &[Self]) -> Vec<String> {
    //     Self::locations(proxies, LType::Datacenter)
    // }

    fn locations(proxies: &[Self], ltype: LType) -> Vec<String> {
        let proxies = proxies.iter();

        let mut locations: Vec<String> = match ltype {
            LType::Country => proxies.map(|proxy| proxy.country().to_string()).collect(),

            LType::City => proxies.map(|proxy| proxy.city().to_string()).collect(),
            // LType::Datacenter => proxies.map(|proxy| proxy.datacenter().to_string()).collect(),
        };

        locations.sort();
        locations.dedup();
        locations
    }
}

#[derive(Clone, Copy)]
enum LType {
    Country,
    City,
    // Datacenter,
}

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Location {
    #[serde(deserialize_with = "make_uniform")]
    country: String,

    #[serde(deserialize_with = "make_uniform")]
    city: String,
    // #[serde(deserialize_with = "make_uniform")]
    // code: String,
}

fn make_uniform<'de, D>(deserializer: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    let mut s = String::deserialize(deserializer)?;

    // split at ',' to remove US state identifiers
    if let Some(index) = s.find(',') {
        s = s.split_at(index).0.to_string();
    }

    // convert to ASCII, remove whitespace, convert to lowercase
    s = deunicode::deunicode(&s).replace(' ', "").to_lowercase();

    Ok(s)
}

#[derive(Clone, PartialEq)]
pub struct Filter {
    city: Option<Vec<String>>,
    country: Option<Vec<String>>,
    // datacenter: Option<Vec<String>>,
    weight: u16,
    offline: Offline,
    style: Style,
    scheme: bool,
    port: bool,
}

pub const DEFAULT_WEIGHT: u16 = 100;

impl Default for Filter {
    fn default() -> Self {
        Self {
            city: None,
            country: None,
            // // datacenter: None,
            weight: DEFAULT_WEIGHT,
            offline: Offline::default(),
            style: Style::default(),
            scheme: false,
            port: false,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Default, PartialEq)]
pub enum Offline {
    #[default]
    Hide,
    Show,
    Only,
}

#[derive(Clone, Copy, Deserialize, Default, PartialEq)]
pub enum Style {
    #[default]
    V4,
    V6,
    Hostname,
}

#[derive(Clone)]
struct Host {
    hostname: String,
    port: u16,
}

impl Host {
    fn new(hostname: String, port: u16) -> Self {
        Self { hostname, port }
    }
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_cities(&mut self, cities: &[String]) -> &mut Self {
        self.city = Some(cities.to_vec());
        self
    }

    pub fn set_countries(&mut self, countries: &[String]) -> &mut Self {
        self.country = Some(countries.to_vec());
        self
    }

    // pub fn set_datacenters(&mut self, datacenters: &[String]) -> &mut Self {
    //     self.datacenter = Some(datacenters.to_vec());
    //     self
    // }

    pub fn set_weight(&mut self, weight: u16) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn set_offline(&mut self, offline: Offline) -> &mut Self {
        self.offline = offline;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn set_scheme(&mut self, scheme: bool) -> &mut Self {
        self.scheme = scheme;
        self
    }

    pub fn set_port(&mut self, port: bool) -> &mut Self {
        self.port = port;
        self
    }

    fn by_country(&self, mut proxies: Vec<Proxy>) -> Vec<Proxy> {
        if let Some(countries) = &self.country {
            proxies.retain(|proxy| countries.contains(&proxy.location.country));
        }

        proxies
    }

    fn by_city(&self, mut proxies: Vec<Proxy>) -> Vec<Proxy> {
        if let Some(cities) = &self.city {
            proxies.retain(|proxy| cities.contains(&proxy.location.city));
        }

        proxies
    }

    // fn by_datacenter(&self, mut proxies: Vec<Proxy>) -> Vec<Proxy> {
    //     if let Some(datacenters) = &self.datacenter {
    //         proxies.retain(|proxy| datacenters.contains(&proxy.location.code));
    //     }

    //     proxies
    // }

    fn by_weight(&self, mut proxies: Vec<Proxy>) -> Vec<Proxy> {
        proxies.retain(|proxy| proxy.weight <= self.weight);

        proxies
    }

    fn by_offline(&self, mut proxies: Vec<Proxy>) -> Vec<Proxy> {
        match self.offline {
            // retain only online proxies
            Offline::Hide => proxies.retain(|proxy| proxy.online),

            // do nothing (retain all proxies)
            Offline::Show => {}

            // retain only offline proxies
            Offline::Only => proxies.retain(|proxy| !proxy.online),
        }

        proxies
    }

    pub fn apply(&self, mut proxies: Vec<Proxy>) -> Vec<String> {
        // filter out unwanted proxies
        proxies = self.by_country(proxies);
        proxies = self.by_city(proxies);
        // proxies = self.by_datacenter(proxies);
        proxies = self.by_weight(proxies);
        proxies = self.by_offline(proxies);

        // build Vec<String>, optionally with scheme and port
        self.add_scheme_and_port(proxies)
    }

    fn add_scheme_and_port(&self, proxies: Vec<Proxy>) -> Vec<String> {
        let proxies = proxies.into_iter();

        // collect proxy IPs/hostnames
        let proxies: Vec<Host> = match self.style {
            Style::Hostname =>
                proxies
                    // some proxies do not have an FQDN hostname,
                    // so this list may be incomplete.
                    // use V4/V6 to get all available proxies.
                    .filter_map(|proxy| {
                        proxy.hostname.map(|hostname| Host::new(hostname, proxy.port))
                    })
                    .collect(),

            Style::V6 => proxies.map(|proxy| Host::new(proxy.ipv6_address, proxy.port)).collect(),

            Style::V4 => proxies.map(|proxy| Host::new(proxy.ipv4_address, proxy.port)).collect(),
        };

        // conditionally add port
        let proxies: Vec<String> = if self.port {
            proxies
                .into_iter()
                .map(|host| format!("{}:{}", host.hostname, host.port))
                .collect()
        } else {
            proxies
                .into_iter()
                .map(|host| host.hostname)
                .collect()
        };

        // conditionally add scheme
        if self.scheme {
            proxies
                .into_iter()
                .map(|proxy| format!("socks5://{proxy}"))
                .collect()
        } else {
            proxies
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ API_URL, Filter, Proxy };
    use reqwest::blocking::Client;

    #[test]
    fn fetch_and_filter() {
        // fetch and deserialize proxies
        let proxies: Vec<Proxy> = Client::new().get(API_URL).send().unwrap().json().unwrap();

        // check if list is non-empty
        assert!(!proxies.is_empty());

        // check if list is non-empty after applying default filters
        assert!(!Filter::default().apply(proxies).is_empty());
    }
}
