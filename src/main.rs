mod api;
mod config;
mod cot;
mod udp;

use crate::{api::start_rest_server, config::Config, udp::start_udp_listener};
use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cfg = Config::from_file("rtak.toml")?;
    info!("Configuration loaded: {:?}", cfg);

    // Clone bind address for UDP listener so it has 'static lifetime
    let udp_bind = cfg.udp_bind.clone();
    let udp_handle = tokio::spawn(async move { start_udp_listener(&udp_bind).await });

    // Run Actix REST server directly
    let rest_result = start_rest_server(&cfg.rest_bind).await;

    if let Err(e) = rest_result {
        eprintln!("REST server failed: {:?}", e);
    }

    udp_handle.abort();

    Ok(())
}
