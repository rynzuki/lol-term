use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("lolt");

    if !dir.exists() {
        fs::create_dir(&dir).expect("Could not create config dir");
    }

    dir.join("config.json")
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub account_name: String,
    pub puuid: String,
    pub summoner_id: String,
}

pub fn load_config() -> Config {
    let path = get_config_path();

    if path.exists() {
        let data = fs::read_to_string(&path).expect("Could not read config file");
        serde_json::from_str(&data).expect("Could not parse config file")
    } else {
        Config::default()
    }
}

pub fn save_config(config: &Config) {
    let path = get_config_path();
    let data = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(path, data).expect("Failed to write config");
}
