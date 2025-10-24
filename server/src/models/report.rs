use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Report {
    id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
