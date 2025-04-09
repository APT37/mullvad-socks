use clap::{ arg, Parser, ValueEnum };
use mvs_lib::{ Filter, Offline, Proxy, Style, API_URL, DEFAULT_WEIGHT };
use rand::Rng;
use reqwest::blocking;
use serde_json::json;
use std::{ error::Error, fmt };

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(version, about)]
struct Config {
    #[arg(short = 'C', long, help = "List of countries (see '-l countries')")]
    countries: Option<String>,

    #[arg(short, long, help = "List of cities (see '-l cities')")]
    cities: Option<String>,

    #[arg(short, long, help = "List of datacenters (see '-l datacenters')")]
    datacenters: Option<String>,

    #[arg(short, long, help = "Weight limit (inclusive)", default_value_t = DEFAULT_WEIGHT)]
    weight: u16,

    #[arg(short, long, help = "Print offline proxies", default_value_t = OfflineStatus::Hide)]
    offline: OfflineStatus,

    #[arg(short, long, help = "Output type", default_value_t = PrintStyle::V4)]
    style: PrintStyle,

    #[arg(short = 'u', long, help = "Prepend scheme ('socks5://')")]
    scheme: bool,

    #[arg(short, long, help = "Append port (':1080')")]
    port: bool,

    #[arg(short, long, help = "List available locations by type")]
    locations: Option<Locator>,

    #[arg(short, long, help = "Format output as JSON")]
    json: bool,

    #[arg(short, long, help = "Print a single, randomly chosen proxy")]
    random: bool,
}

#[derive(Clone, ValueEnum)]
enum OfflineStatus {
    Hide,
    Show,
    Only,
}

impl OfflineStatus {
    fn to_lib_type(&self) -> Offline {
        match self {
            Self::Hide => Offline::Hide,
            Self::Show => Offline::Show,
            Self::Only => Offline::Only,
        }
    }
}

impl fmt::Display for OfflineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offline = match self {
            Self::Hide => "hide",
            Self::Show => "show",
            Self::Only => "only",
        };

        write!(f, "{offline}")
    }
}

#[derive(Clone, ValueEnum)]
enum PrintStyle {
    V4,
    V6,
    Hostname,
}

impl PrintStyle {
    fn to_lib_type(&self) -> Style {
        match self {
            Self::V4 => Style::V4,
            Self::V6 => Style::V6,
            Self::Hostname => Style::Hostname,
        }
    }
}

impl fmt::Display for PrintStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let offline = match self {
            Self::V4 => "v4",
            Self::V6 => "v6",
            Self::Hostname => "hostname",
        };

        write!(f, "{offline}")
    }
}

#[derive(Clone, ValueEnum)]
enum Locator {
    Countries,
    Cities,
    Datacenters,
}

impl Config {
    fn to_filter(&self) -> Filter {
        fn split_list(list: &str) -> Vec<String> {
            list.trim().split(',').map(String::from).collect()
        }

        let mut filter = Filter::new();

        if let Some(countries) = &self.countries {
            filter.set_countries(&split_list(countries));
        }

        if let Some(cities) = &self.cities {
            filter.set_cities(&split_list(cities));
        }

        if let Some(datacenters) = &self.datacenters {
            filter.set_datacenters(&split_list(datacenters));
        }

        filter
            .set_weight(self.weight)
            .set_offline(self.offline.to_lib_type())
            .set_style(self.style.to_lib_type())
            .set_scheme(self.scheme)
            .set_port(self.port);

        filter
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    let proxies: Vec<_> = blocking::get(API_URL)?.json()?;

    let list = if let Some(location_type) = config.locations {
        match location_type {
            Locator::Countries => Proxy::countries(&proxies),
            Locator::Cities => Proxy::cities(&proxies),
            Locator::Datacenters => Proxy::datacenters(&proxies),
        }
    } else {
        let filtered = config.to_filter().apply(proxies).into_iter().collect::<Vec<String>>();

        if config.random && !filtered.is_empty() {
            vec![filtered[rand::rng().random_range(0..filtered.len())].clone()]
        } else {
            filtered
        }
    };

    println!("{}", {
        if config.json { json!(list).to_string() } else { list.join("\n") }
    });

    Ok(())
}
