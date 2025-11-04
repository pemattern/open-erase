use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        let token_type = String::from("Bearer");
        Self {
            access_token,
            refresh_token,
            token_type,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

impl RefreshResponse {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        let token_type = String::from("Bearer");
        Self {
            access_token,
            refresh_token,
            token_type,
        }
    }
}
