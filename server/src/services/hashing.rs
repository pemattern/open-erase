use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::error::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct HashingService;

impl HashingService {
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
