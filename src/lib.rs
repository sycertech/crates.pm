use color_eyre::{eyre::eyre, Result};
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

use config::CONFIG;
use flate2::bufread::GzDecoder;
use futures::channel::mpsc;
use futures::stream::StreamExt;
use futures_timer::Delay;

use cargo_toml::Manifest;
use meilisearch_sdk::Client;
use serde::{Deserialize, Serialize};
use tar::Archive;

pub mod backoff;
pub mod config;

#[derive(Debug, Deserialize)]
pub struct CrateInfo {
    pub name: String,
    pub vers: String,
}

#[derive(Debug, Serialize)]
pub struct CompleteCrateInfos {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub readme: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadsCrateInfos {
    pub name: String,
    downloads: u64,
}

#[tracing::instrument(skip_all)]
pub async fn retrieve_crate_toml(info: &CrateInfo) -> Result<CompleteCrateInfos> {
    let url = format!(
        "https://static.crates.io/crates/{name}/{name}-{version}.crate",
        name = info.name,
        version = info.vers,
    );

    let mut result = None;
    for multiplier in backoff::new().take(10) {
        match reqwest::get(&url).await {
            Ok(res) => {
                result = Some(res);
                break;
            }
            Err(e) => {
                let dur = Duration::from_secs(1) * (multiplier + 1);
                tracing::warn!("error downloading {} {} retrying in {:.2?}", url, e, dur);
                let _ = Delay::new(dur).await;
            }
        }
    }

    let res = match result {
        Some(res) => res,
        None => return Err(eyre!("Could not download {}", url)),
    };

    if !res.status().is_success() {
        let body = res.text().await?;
        return Err(eyre!("Could not download {}: {}", url, body));
    }

    let bytes = res.bytes().await?;
    let archive = GzDecoder::new(&bytes[..]);
    let mut tar = Archive::new(archive);

    let mut toml_infos = None;
    let mut readme = None;

    for res in tar.entries()? {
        // stop early if we found both files
        if toml_infos.is_some() && readme.is_some() {
            break;
        }

        let mut entry = res?;
        let path = entry.path()?;
        let file_name = path.file_name().and_then(|s| s.to_str());

        match file_name {
            Some("Cargo.toml") if toml_infos.is_none() => {
                let mut content = Vec::new();
                entry.read_to_end(&mut content)?;

                let manifest = Manifest::from_slice(&content)?;
                let package = match manifest.package {
                    Some(package) => package,
                    None => break,
                };

                let name = info.name.clone();
                let description = package.description.unwrap_or_default();
                let keywords = package.keywords;
                let categories = package.categories;
                let version = info.vers.clone();

                toml_infos = Some((name, description, keywords, categories, version));
            }
            Some("README.md") if readme.is_none() => {
                let mut content = String::new();
                entry.read_to_string(&mut content)?;

                let options = comrak::ComrakOptions::default();
                let html = comrak::markdown_to_html(&content, &options);

                let document = scraper::Html::parse_document(&html);
                let html = document.root_element();
                let text = html.text().collect();

                readme = Some(text);
            }
            _ => (),
        }
    }

    match (toml_infos, readme) {
        (Some((name, description, keywords, categories, version)), readme) => {
            Ok(CompleteCrateInfos {
                name,
                description: description.get().unwrap_or(&"".to_string()).to_string(),
                keywords: keywords.get().unwrap_or(&vec![]).to_vec(),
                categories: categories.get().unwrap_or(&vec![]).to_vec(),
                readme: readme.unwrap_or_default(),
                version,
            })
        }
        (None, _) => Err(eyre!("No Cargo.toml found in this crate")),
    }
}

#[tracing::instrument(skip_all)]
pub async fn chunk_info_to_meili<T: Serialize + Send + Sync + 'static>(
    client: Arc<Client>,
    receiver: mpsc::Receiver<T>,
) -> Result<()> {
    let index = client.index(CONFIG.meili_index_uid.clone());

    let mut js = JoinSet::new();

    let mut chunk_count = 0;
    let mut receiver = receiver.chunks(250);
    while let Some(chunk) = receiver.next().await {
        tracing::debug!("chunk {chunk_count} len: {}", chunk.len());
        chunk_count += 1;

        let index = index.clone();
        let client = client.clone();
        js.spawn(async move {
            let task = match index.add_or_update(&chunk, Some("name")).await {
                Ok(task) => task,
                Err(e) => {
                    tracing::error!("error adding chunk to meili: {}", e);
                    return;
                }
            };
            let task_id = task.task_uid;
            tracing::debug!("sending chunk {chunk_count} (task: {task_id}) {task:?}");

            match client
                .wait_for_task(task, None, Some(Duration::from_secs(120)))
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::warn!("error waiting for task: {}", e);
                    return;
                }
            };
            tracing::debug!("task for chunk {chunk_count} passed (task: {task_id})");
        });
    }

    tracing::info!("waiting for meili to finish {} tasks", js.len());
    while (js.join_next().await).is_some() {}

    Ok(())
}

/// Create a meilisearch client
#[tracing::instrument(skip_all)]
pub fn create_meilisearch_client() -> Arc<Client> {
    Arc::new(Client::new(
        CONFIG.meili_host_url.clone(),
        Some(CONFIG.meili_api_key.clone()),
    ))
}

/// Initialize logging for the application
#[tracing::instrument(skip_all)]
pub fn init_logging() {
    color_eyre::install().expect("failed to install color_eyre");

    use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

    Registry::default()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,html5ever=info,isahc=info".into()),
        )
        .with(fmt::layer())
        .init();
}
