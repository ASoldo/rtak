use actix::Actor;

mod api;
mod broadcaster;
mod config;
mod cot;
mod udp;

use crate::broadcaster::Broadcaster;
use crate::{api::start_rest_server, config::Config, udp::start_udp_listener};
use tracing::{Level, info};
use tracing_subscriber;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let cfg = Config::from_file("rtak.toml")?;
            info!("Configuration loaded: {:?}", cfg);

            let broadcaster = Broadcaster::new().start();

            let udp_broadcaster = broadcaster.clone();
            let udp_bind = cfg.udp_bind.clone();
            let udp_handle =
                tokio::spawn(async move { start_udp_listener(&udp_bind, udp_broadcaster).await });

            let rest_result = start_rest_server(&cfg.rest_bind, broadcaster.clone()).await;

            if let Err(e) = rest_result {
                eprintln!("REST server failed: {:?}", e);
            }

            udp_handle.abort();

            Ok::<(), anyhow::Error>(())
        })
        .await
}
