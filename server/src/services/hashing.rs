use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::services::ServerError;

#[derive(Clone)]
pub struct HashingService;

impl HashingService {
    pub fn hash_password(&self, password: &str) -> Result<String, ServerError> {
        let argon2 = Argon2::default();
        let salt_string = SaltString::generate(&mut OsRng);
        argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map(|hash| hash.to_string())
            .map_err(ServerError::Hash)
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> Result<(), ServerError> {
        let parsed_hash = match PasswordHash::new(password_hash) {
            Ok(hash) => hash,
            Err(err) => return Err(ServerError::Hash(err)),
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(ServerError::Hash)
    }
}
