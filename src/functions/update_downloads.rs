use std::ffi::OsStr;
use std::sync::Arc;

use flate2::read::GzDecoder;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};
use meilisearch_sdk::Client;
use tar::Archive;

use crate::{chunk_info_to_meili, DownloadsCrateInfos};
use color_eyre::Result;

#[tracing::instrument]
pub async fn update_downloads(client: Arc<Client>) -> Result<()> {
    let (cinfos_sender, cinfos_receiver) = mpsc::channel(100);

    let downloads_infos = stream::iter(crates_downloads_infos().await?);

    let publish_handler = tokio::spawn(chunk_info_to_meili(client, cinfos_receiver));

    StreamExt::zip(downloads_infos, stream::repeat(cinfos_sender))
        .for_each_concurrent(Some(8), |(info, mut sender)| async move {
            sender.send(info).await.unwrap()
        })
        .await;

    publish_handler.await??;

    Ok(())
}

#[tracing::instrument]
async fn crates_downloads_infos() -> Result<Vec<DownloadsCrateInfos>> {
    let body = reqwest::get("https://static.crates.io/db-dump.tar.gz")
        .await?
        .bytes()
        .await?;
    let gz = GzDecoder::new(&body[..]);
    let mut tar = Archive::new(gz);

    let mut entry = None;
    for result in tar.entries()? {
        let e = result?;
        if e.path()?.file_name() == Some(OsStr::new("crates.csv")) {
            entry = Some(e);
            break;
        }
    }

    let entry = match entry {
        Some(entry) => entry,
        None => return Ok(Vec::new()),
    };

    let mut downloads_infos = Vec::new();

    let mut rdr = csv::Reader::from_reader(entry);
    for result in rdr.deserialize() {
        let info: DownloadsCrateInfos = result?;
        downloads_infos.push(info);
    }

    Ok(downloads_infos)
}
