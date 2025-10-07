use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::services::ServiceError;

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let argon2 = Argon2::default();
    let salt_string = SaltString::generate(&mut OsRng);
    argon2
        .hash_password(password.as_bytes(), &salt_string)
        .map(|hash| hash.to_string())
        .map_err(ServiceError::Hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), ServiceError> {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(err) => return Err(ServiceError::Hash(err)),
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|err| match err {
            argon2::password_hash::Error::Password => ServiceError::Auth,
            _ => ServiceError::Hash(err),
        })
}
