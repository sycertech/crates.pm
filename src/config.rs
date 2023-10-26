use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The MeiliSearch host URL
    pub meili_host_url: String,
    /// The MeiliSearch index UID
    pub meili_index_uid: String,
    /// The MeiliSearch API key
    pub meili_api_key: String,
}

impl Config {
    /// Create a new `Config`
    pub fn new() -> Result<Self> {
        let config = envy::from_env::<Self>()?;

        Ok(config)
    }
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
    &CONFIG
}
