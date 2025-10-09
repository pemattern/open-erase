use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::User;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub uuid: Uuid,
    pub email: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct UserPasswordHash {
    pub uuid: Uuid,
    pub password_hash: String,
}

impl From<User> for UserPasswordHash {
    fn from(value: User) -> Self {
        Self {
            uuid: value.uuid,
            password_hash: value.password_hash,
        }
    }
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            uuid: value.uuid,
            email: value.email,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
