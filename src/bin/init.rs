use std::path::Path;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use color_eyre::eyre::Result;
use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::{self, StreamExt};
use meilisearch_sdk::{Client, Settings};
use search_crates_pm::config::CONFIG;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use walkdir::WalkDir;

use search_crates_pm::{
    chunk_info_to_meili, create_meilisearch_client, init_logging, retrieve_crate_toml, CrateInfo,
};

#[tracing::instrument]
async fn process_file(entry: walkdir::DirEntry) -> Result<Option<CrateInfo>> {
    if entry.file_type().is_file() {
        let file = fs::File::open(entry.path()).await?;
        let file = BufReader::new(file);
        let mut lines = file.lines();

        let mut last = None;
        while let Some(line) = lines.next_line().await? {
            last = Some(line);
        }

        let last_line = match last {
            Some(line) => line,
            None => return Ok(None),
        };

        let info: CrateInfo = match serde_json::from_str(&last_line) {
            Ok(info) => info,
            Err(_) => return Ok(None),
        };

        return Ok(Some(info));
    }

    Ok(None)
}

#[tracing::instrument(skip_all)]
async fn crates_infos<P: AsRef<Path>>(
    mut sender: mpsc::Sender<CrateInfo>,
    crates_io_index: P,
) -> Result<()> {
    let walkdir = WalkDir::new(crates_io_index).contents_first(true);

    let count = Arc::new(AtomicUsize::new(0));
    for result in walkdir {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => {
                tracing::error!("{}", e);
                continue;
            }
        };

        let count = count.clone();
        let mut sender = sender.clone();
        tokio::spawn(async move {
            match process_file(entry).await {
                Ok(Some(info)) => {
                    let i = count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                    tracing::debug!(
                        name = info.name,
                        version = info.vers,
                        count = i + 1,
                        "sending crate"
                    );
                    if let Err(e) = sender.send(info).await {
                        tracing::error!("failed to send crate info: {:#?}", e);
                    }
                }
                Ok(None) => (),
                Err(e) => tracing::error!("error when processing file: {}", e),
            }
        });
    }

    sender.flush().await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn init_index(client: &Client) -> Result<()> {
    let task = client
        .create_index(CONFIG.meili_index_uid.clone(), Some("name"))
        .await?;
    let res = client.wait_for_task(task, None, None).await?;

    tracing::info!("{res:#?}");

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn init_settings(client: &Client) -> Result<()> {
    let index = client.index(CONFIG.meili_index_uid.clone());

    let settings = Settings {
        searchable_attributes: Some(vec![
            "name".to_string(),
            "description".to_string(),
            "keywords".to_string(),
            "categories".to_string(),
            "readme".to_string(),
        ]),
        sortable_attributes: Some(vec!["downloads".to_string()]),
        ..Default::default()
    };

    let task = index.set_settings(&settings).await?;
    let res = client.wait_for_task(task, None, None).await?;

    tracing::info!("{res:#?}");

    Ok(())
}

// git clone --depth=1 https://github.com/rust-lang/crates.io-index.git
// https://static.crates.io/crates/{crate}/{crate}-{version}.crate

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let (infos_sender, infos_receiver) = mpsc::channel(10_000);
    let (cinfos_sender, cinfos_receiver) = mpsc::channel(10_000);

    let client = create_meilisearch_client();

    init_index(&client).await?;
    init_settings(&client).await?;

    let retrieve_handler = tokio::spawn(crates_infos(infos_sender, "crates.io-index/"));

    let publish_handler = tokio::spawn(chunk_info_to_meili(client.clone(), cinfos_receiver));

    let retrieve_toml = StreamExt::zip(infos_receiver, stream::repeat(cinfos_sender))
        .for_each_concurrent(Some(250), |(info, mut sender)| async move {
            let _ = tokio::spawn(async move {
                match retrieve_crate_toml(&info).await {
                    Ok(cinfo) => match sender.send(cinfo).await {
                        Ok(_) => (),
                        Err(e) => tracing::error!("failed to send crate info: {:#?}", e),
                    },
                    Err(e) => tracing::error!("{:?} {}", info, e),
                }
            })
            .await;
        });

    retrieve_toml.await;
    retrieve_handler.await??;
    publish_handler.await??;

    Ok(())
}
