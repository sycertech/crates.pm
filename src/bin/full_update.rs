use std::ffi::OsStr;

use flate2::read::GzDecoder;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};
use tar::Archive;

use color_eyre::Result;
use search_crates_pm::{
    chunk_downloads_crates_info_to_meili, create_meilisearch_client, init_logging,
    DownloadsCrateInfos,
};

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

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let (cinfos_sender, cinfos_receiver) = mpsc::channel(100);

    let downloads_infos = stream::iter(crates_downloads_infos().await?);

    let client = create_meilisearch_client();
    let publish_handler = tokio::spawn(chunk_downloads_crates_info_to_meili(
        client,
        cinfos_receiver,
    ));

    StreamExt::zip(downloads_infos, stream::repeat(cinfos_sender))
        .for_each_concurrent(Some(8), |(info, mut sender)| async move {
            sender.send(info).await.unwrap()
        })
        .await;

    publish_handler.await??;

    Ok(())
}
