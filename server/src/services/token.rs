use std::time::Duration;

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::Config, error::ServiceResult};

#[derive(Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct TokenService;

impl TokenService {
    pub fn generate(&self, user_uuid: Uuid, config: &Config) -> ServiceResult<String> {
        let sub = user_uuid.to_string();
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let access_token_expires_at = now + Duration::from_secs(config.access_token_validity_secs);
        let secret = config.secret.as_bytes();
        let key = EncodingKey::from_secret(secret);
        let exp = access_token_expires_at.timestamp() as usize;
        let iss = config.issuer.clone();
        let claims = Claims { sub, exp, iat, iss };
        let access_token = encode(&Header::default(), &claims, &key)?;
        Ok(access_token)
    }

    pub fn decode(&self, jwt: &str, config: &Config) -> ServiceResult<Claims> {
        let key = DecodingKey::from_secret(config.secret.as_bytes());
        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[&config.issuer]);
        let claims =
            decode::<Claims>(jwt, &key, &Validation::new(jsonwebtoken::Algorithm::HS256))?.claims;
        Ok(claims)
    }
}
