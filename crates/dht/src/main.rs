use std::{collections::HashSet, str::FromStr, time::Duration};

use anyhow::Context;
use dht::{dht::Dht, id20::Id20};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let info_hash = Id20::from_str("64a980abe6e448226bb930ba061592e44c3781a1").unwrap();
    let dht = Dht::new(&["dht.transmissionbt.com:6881", "dht.libtorrent.org:25401"])
        .await
        .context("error initializing dht")?;
    let mut stream = dht.get_peers(info_hash).await;
    let mut seen = HashSet::new();
    while let Some(peer) = stream.next().await {
        if seen.insert(peer) {
            log::info!("peer found: {}", peer)
        }
    }
    Ok(())
}
