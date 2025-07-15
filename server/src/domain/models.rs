use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub password_hash: String,
    pub created_on: DateTime<Local>,
    pub updated_on: DateTime<Local>,
}

impl User {
    pub fn new(name: String, password_hash: String) -> Self {
        let uuid = Uuid::now_v7();
        let created_on = chrono::offset::Local::now();
        let updated_on = created_on;
        Self {
            uuid,
            name,
            password_hash,
            created_on,
            updated_on,
        }
    }
}
