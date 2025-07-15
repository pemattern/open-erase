use argon2::PasswordHasher;
use argon2::{Argon2, password_hash::SaltString};
use uuid::Uuid;

pub fn hash_password<S: Into<String>>(password: S) -> String {
    let argon2 = Argon2::default();
    let salt_string = SaltString::encode_b64(Uuid::now_v7().as_bytes()).unwrap();
    let password_hash = argon2
        .hash_password(password.into().as_bytes(), &salt_string)
        .unwrap();
    password_hash.to_string()
}
