use chrono::{DateTime, Local};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}
