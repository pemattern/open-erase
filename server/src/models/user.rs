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
