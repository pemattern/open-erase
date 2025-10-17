use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};
use uuid::Uuid;

use crate::{
    error::{ServiceError, ServiceResult},
    models::User,
    repositories::{refresh_token::RefreshTokenRepository, user::UserRepository},
};

const CONFIG_FILE_PATH: &str = "Server.toml";
const ISSUER: &str = "open-erase";
const ACCESS_TOKEN_VALIDITY_SECS: Duration = Duration::from_secs(60 * 15);
const KEY_LENGTH: usize = 32;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| Argon2::default());
static ENCRYPTION_KEY: LazyLock<[u8; KEY_LENGTH]> =
    LazyLock::new(|| generate_byte_key::<KEY_LENGTH>());

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let access_token_expires_at = now + ACCESS_TOKEN_VALIDITY_SECS;
        let sub = user_id.to_string();
        let exp = access_token_expires_at.timestamp() as usize;
        let iat = now.timestamp() as usize;
        let iss = String::from(ISSUER);
        Self { sub, iss, exp, iat }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    refresh_token_repository: Arc<dyn RefreshTokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    ) -> Self {
        Self {
            user_repository,
            refresh_token_repository,
        }
    }

    pub fn generate_access_token(&self, user: &User) -> ServiceResult<String> {
        let claims = Claims::new(user.id);
        let key = EncodingKey::from_secret(&*ENCRYPTION_KEY);
        let access_token = encode(&Header::default(), &claims, &key)?;
        Ok(access_token)
    }

    pub fn validate_access_token(&self, access_token: &str) -> ServiceResult<Claims> {
        let key = DecodingKey::from_secret(&*ENCRYPTION_KEY);
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[ISSUER]);
        let claims = decode::<Claims>(access_token, &key, &validation)?.claims;
        Ok(claims)
    }

    pub async fn generate_refresh_token(&self, user: &User) -> ServiceResult<String> {
        let refresh_token_bytes = generate_byte_key::<KEY_LENGTH>();
        let refresh_token = BASE64_URL_SAFE_NO_PAD.encode(refresh_token_bytes);
        let refresh_token_hash = generate_hash(&refresh_token)?;
        self.refresh_token_repository
            .create(user.id, refresh_token_hash)
            .await?;
        Ok(BASE64_URL_SAFE_NO_PAD.encode(refresh_token))
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> ServiceResult<()> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(ServiceError::Hash)?;
        ARGON2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(ServiceError::Hash)
    }
}

fn generate_hash(password: &str) -> ServiceResult<String> {
    let salt_string = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_bytes(), &salt_string)
        .map(|hash| hash.to_string())
        .map_err(ServiceError::Hash)
}

fn generate_byte_key<const N: usize>() -> [u8; N] {
    let mut key = [0u8; N];
    getrandom::fill(&mut key).unwrap();
    key
}
