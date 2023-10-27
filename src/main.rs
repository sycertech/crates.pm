use std::{sync::Arc, time::Duration};

use clap::Parser;
use clokwerk::{AsyncScheduler, Job, TimeUnits};
use color_eyre::eyre::Result;
use meilisearch_sdk::Client;
use search_crates_pm::{
    create_meilisearch_client,
    functions::{init::init, live::live, update_downloads::update_downloads},
};
use tokio::time::sleep;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
    /// The parsed command
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Loads all crates
    Init,

    /// Runs update tasks on a schedule
    Run,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().expect("failed to install color_eyre");

    Registry::default()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,html5ever=info,isahc=info".into()),
        )
        .with(fmt::layer())
        .init();

    let client = create_meilisearch_client();

    match Opt::parse().command {
        Some(Command::Init) => init(client).await,
        _ => run(client).await,
    }
}

async fn run(client: Arc<Client>) -> Result<()> {
    let mut scheduler = AsyncScheduler::with_tz(chrono::Utc);

    let client_clone = client.clone();
    scheduler.every(1.day()).at("2:30 AM").run(move || {
        let client = client_clone.clone();
        async move {
            update_downloads(client).await.unwrap();
        }
    });

    let client_clone = client.clone();
    scheduler.every(10.minutes()).run(move || {
        let client = client_clone.clone();
        async move {
            live(client).await.unwrap();
        }
    });

    live(client).await.unwrap();

    loop {
        scheduler.run_pending().await;
        sleep(Duration::from_millis(500)).await;
    }
}
