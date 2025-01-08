use axum::{routing::get, Router};
use log::{error, info};
use proxydata::ProxyData;
use std::{io, net::Ipv4Addr, thread, time::Duration};
use tokio::{
    net::TcpListener,
    sync::watch::{channel, Sender},
};

mod env;
mod filter;
mod locations;
mod proxydata;
mod version;

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    // populate channel with empty ProxyData
    let (tx, rx) = channel(ProxyData::default());

    // start updater job in background thread
    thread::Builder::new()
        .name("update_thread".to_string())
        .spawn(|| update(tx))?;

    let host = env::lookup("HOST", Ipv4Addr::new(127, 0, 0, 1));
    let port = env::lookup("PORT", 8080);

    let address = format!("{host}:{port}");

    info!("binding to {address}");

    let listener = TcpListener::bind(address).await?;

    let app = Router::new()
        .route("/proxies", get(filter::filter))
        .route("/locations", get(locations::locations))
        .route("/version", get(version::version))
        .with_state(rx);

    axum::serve(listener, app).await
}

#[allow(clippy::needless_pass_by_value)]
fn update(tx: Sender<ProxyData>) {
    loop {
        // fetch socks proxies from API
        match ProxyData::new() {
            Ok(data) => tx.send(data).unwrap(),
            Err(err) => error!("{err}"),
        }

        // wait for a set interval before next fetch
        thread::sleep(Duration::from_secs(300));
    }
}
