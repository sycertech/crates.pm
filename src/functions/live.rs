use std::io::BufReader;
use std::sync::Arc;

use atom_syndication::Feed;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};

use crate::{chunk_info_to_meili, retrieve_crate_toml, CrateInfo};
use color_eyre::Result;
use meilisearch_sdk::Client;

#[tracing::instrument]
pub async fn live(client: Arc<Client>) -> Result<()> {
    let (infos_sender, infos_receiver) = mpsc::channel(10_000);
    let (cinfos_sender, cinfos_receiver) = mpsc::channel(10_000);

    let retrieve_handler = tokio::spawn(crates_infos(infos_sender));

    let publish_handler = tokio::spawn(chunk_info_to_meili(client, cinfos_receiver));

    StreamExt::zip(infos_receiver, stream::repeat(cinfos_sender))
        .for_each_concurrent(Some(8), |(info, mut sender)| async move {
            match retrieve_crate_toml(&info).await {
                Ok(cinfo) => sender.send(cinfo).await.unwrap(),
                Err(e) => tracing::info!("{:?} {}", info, e),
            }
        })
        .await;

    retrieve_handler.await??;
    publish_handler.await??;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn crates_infos(mut sender: mpsc::Sender<CrateInfo>) -> Result<()> {
    let body = reqwest::get("https://docs.rs/releases/feed")
        .await?
        .bytes()
        .await?;
    let feed = Feed::read_from(BufReader::new(&body[..])).unwrap();

    for entry in feed.entries() {
        // urn:docs-rs:hello_exercism:0.2.8
        let name = match entry.id().split(':').nth(2) {
            Some(name) => name.to_string(),
            None => continue,
        };

        let vers = match entry.id().split(':').nth(3) {
            Some(vers) => vers.to_string(),
            None => continue,
        };

        let info = CrateInfo { name, vers };

        if let Err(e) = sender.send(info).await {
            tracing::info!("{}", e);
        }
    }

    Ok(())
}
