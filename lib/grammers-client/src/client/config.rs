use std::sync::LazyLock;

use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    pub upload_worker_count: usize,
    pub big_file_size: usize,
}

pub static APP_CONFIG: LazyLock<ClientConfig> = LazyLock::new(load);

fn load() -> ClientConfig {
    let cfg_file_path = dirs::config_local_dir()
        .unwrap()
        .join("grammers")
        .join("cfg.toml");

    Config::builder()
        .add_source(config::File::from(cfg_file_path))
        .build()
        .and_then(Config::try_deserialize)
        .expect("Failed to load configuration")
}
