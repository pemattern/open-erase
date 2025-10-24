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
    error::ServiceResult,
    models::{RefreshToken, User},
    repositories::{refresh_token::RefreshTokenRepository, user::UserRepository},
};

const ISSUER: &str = "open-erase";
const ACCESS_TOKEN_VALIDITY_DURATION: Duration = Duration::from_secs(60 * 15); // 15 minutes
const KEY_LENGTH: usize = 32;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(Argon2::default);
static ENCRYPTION_KEY: LazyLock<[u8; KEY_LENGTH]> = LazyLock::new(generate_byte_key::<KEY_LENGTH>);

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
        let access_token_expires_at = now + ACCESS_TOKEN_VALIDITY_DURATION;
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

    pub async fn validate_basic_auth(
        &self,
        email: &str,
        password: &str,
    ) -> ServiceResult<Option<User>> {
        if let Some(user) = self.user_repository.find_by_email(email).await?
            && is_valid_password(password, &user.password_hash)
        {
            return Ok(Some(user));
        };
        Ok(None)
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> ServiceResult<String> {
        let claims = Claims::new(user_id);
        let key = EncodingKey::from_secret(&*ENCRYPTION_KEY);
        Ok(encode(&Header::default(), &claims, &key)?)
    }

    pub fn get_valid_access_token_claims(&self, access_token: &str) -> Option<Claims> {
        let key = DecodingKey::from_secret(&*ENCRYPTION_KEY);
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[ISSUER]);
        decode::<Claims>(access_token, &key, &validation)
            .ok()
            .map(|token_data| token_data.claims)
    }

    pub async fn generate_refresh_token_from_login(&self, user_id: Uuid) -> ServiceResult<String> {
        self.generate_refresh_token(user_id, None).await
    }

    pub async fn cycle_refresh_token(&self, refresh_token: &RefreshToken) -> ServiceResult<String> {
        let _ = self
            .refresh_token_repository
            .mark_as_used(refresh_token.id)
            .await?;

        self.generate_refresh_token(refresh_token.user_id, Some(refresh_token.id))
            .await
    }

    pub async fn mark_refresh_token_as_used(
        &self,
        refresh_token: &RefreshToken,
    ) -> ServiceResult<RefreshToken> {
        let refresh_token = self
            .refresh_token_repository
            .mark_as_used(refresh_token.id)
            .await?;
        Ok(refresh_token)
    }

    async fn generate_refresh_token(
        &self,
        user_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> ServiceResult<String> {
        let opaque_token_bytes = generate_byte_key::<KEY_LENGTH>();
        let opaque_token_raw = BASE64_URL_SAFE_NO_PAD.encode(opaque_token_bytes);
        let opaque_token_hash = generate_hash(&opaque_token_raw)?;
        let refresh_token = self
            .refresh_token_repository
            .create(user_id, parent_id, opaque_token_hash)
            .await?;
        let composite_refresh_token = format!("{}.{}", refresh_token.id, opaque_token_raw);
        Ok(composite_refresh_token)
    }

    pub async fn get_valid_refresh_token(
        &self,
        composite_refresh_token: &str,
    ) -> ServiceResult<Option<RefreshToken>> {
        if let Some((refresh_token_id_raw, opaque_token_raw)) =
            composite_refresh_token.split_once('.')
            && let Ok(refresh_token_id) = Uuid::parse_str(refresh_token_id_raw)
            && let Some(refresh_token) = self
                .refresh_token_repository
                .find_by_id(refresh_token_id)
                .await?
            && refresh_token.is_valid()
            && is_valid_password(opaque_token_raw, &refresh_token.opaque_token_hash)
        {
            return Ok(Some(refresh_token));
        };
        Ok(None)
    }
}

fn is_valid_password(password: &str, password_hash: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(password_hash)
        && ARGON2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    {
        return true;
    }
    false
}

fn generate_hash(password: &str) -> ServiceResult<String> {
    let salt_string = SaltString::generate(&mut OsRng);
    Ok(ARGON2
        .hash_password(password.as_bytes(), &salt_string)
        .map(|hash| hash.to_string())?)
}

fn generate_byte_key<const N: usize>() -> [u8; N] {
    let mut key = [0u8; N];
    getrandom::fill(&mut key).unwrap();
    key
}
