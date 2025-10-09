use chrono::{DateTime, Local};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl User {
    pub fn mock() -> Self {
        User {
            uuid: Uuid::default(),
            email: String::from("test@mail.com"),
            password_hash: String::from("hash.."),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}
