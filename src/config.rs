#[derive(Clone)]
pub struct Config {
    pub secret: String,
    pub issuer: String,
    pub access_token_validity_secs: u64,
    pub refresh_token_validity_secs: u64,
}
