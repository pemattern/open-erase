use std::{fs, sync::LazyLock};

use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "Server.toml";
const DEFAULT_ISSUER: &str = "open-erase";
const DEFAULT_ACCESS_TOKEN_VALIDITY_SECS: u64 = 60 * 60 * 24 * 7;

pub static SERVER_CONFIG: LazyLock<Config> = LazyLock::new(deserialize_config);

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub secret: String,
    pub issuer: String,
    pub access_token_validity_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        let encryption_key = generate_byte_key(32);
        let issuer = String::from(DEFAULT_ISSUER);
        let access_token_validity_secs = DEFAULT_ACCESS_TOKEN_VALIDITY_SECS;
        Self {
            secret: encryption_key,
            issuer,
            access_token_validity_secs,
        }
    }
}

fn deserialize_config() -> Config {
    let Ok(file_string) = fs::read_to_string(CONFIG_FILE_PATH) else {
        return Config::default();
    };
    let Ok(config) = toml::from_str(&file_string) else {
        return Config::default();
    };
    config
}

fn generate_byte_key(length: usize) -> String {
    let mut key = vec![0u8; length];
    getrandom::fill(&mut key).expect("unable to generate random encryption key");
    hex::encode(&key)
}
