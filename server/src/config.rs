use std::sync::LazyLock;

use serde::Deserialize;

const DEFAULT_ISSUER: &str = "open-erase";
const DEFAULT_ACCESS_TOKEN_VALIDITY_SECS: u64 = 500;
const DEFAULT_REFRESH_TOKEN_VALIDITY_SECS: u64 = 3600 * 24 * 7;

static CONFIG: LazyLock<Config> = LazyLock::new(|| deserialize_config());

#[derive(Clone, Deserialize)]
pub struct Config {
    pub encryption_key: String,
    pub issuer: String,
    pub access_token_validity_secs: u64,
    pub refresh_token_validity_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        let encryption_key = generate_encryption_key(32);
        let issuer = DEFAULT_ISSUER.to_string();
        let access_token_validity_secs = DEFAULT_ACCESS_TOKEN_VALIDITY_SECS;
        let refresh_token_validity_secs = DEFAULT_REFRESH_TOKEN_VALIDITY_SECS;
        Self {
            encryption_key,
            issuer,
            access_token_validity_secs,
            refresh_token_validity_secs,
        }
    }
}

fn deserialize_config() -> Config {
    Config::default()
}

fn generate_encryption_key(length: usize) -> String {
    let mut key = vec![0u8; length];
    getrandom::fill(&mut key).expect("unable to generate random encryption key");
    hex::encode(&key)
}
