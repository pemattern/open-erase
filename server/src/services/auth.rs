use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

use crate::{
    config::Config,
    error::{ServiceError, ServiceResult},
    models::User,
    repositories::user::UserRepository,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: Uuid, config: &Config) -> Self {
        let now = Utc::now();
        let access_token_expires_at = now + Duration::from_secs(config.access_token_validity_secs);
        let sub = user_id.to_string();
        let exp = access_token_expires_at.timestamp() as usize;
        let iat = now.timestamp() as usize;
        let iss = config.issuer.clone();
        Self { sub, iss, exp, iat }
    }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl AuthService {
    pub fn generate_access_token(&self, user: User, config: &Config) -> ServiceResult<String> {
        let claims = Claims::new(user.id, config);
        let secret = config.secret.as_bytes();
        let key = EncodingKey::from_secret(secret);
        let access_token = encode(&Header::default(), &claims, &key)?;
        Ok(access_token)
    }

    pub fn validate_access_token(
        &self,
        access_token: &str,
        config: &Config,
    ) -> ServiceResult<Claims> {
        let key = DecodingKey::from_secret(config.secret.as_bytes());
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[&config.issuer]);
        let claims = decode::<Claims>(
            access_token,
            &key,
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?
        .claims;
        Ok(claims)
    }

    pub fn hash_password(&self, password: &str) -> ServiceResult<String> {
        let argon2 = Argon2::default();
        let salt_string = SaltString::generate(&mut OsRng);
        argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map(|hash| hash.to_string())
            .map_err(ServiceError::Hash)
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> ServiceResult<()> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(ServiceError::Hash)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(ServiceError::Hash)
    }
}
