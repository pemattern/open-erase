use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::DatabaseResult, models::RefreshToken};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DatabaseResult<Option<RefreshToken>>;
    async fn create(&self) -> DatabaseResult<RefreshToken>;
    async fn update(&self, id: Uuid) -> DatabaseResult<RefreshToken>;
    async fn delete(&self, id: Uuid) -> DatabaseResult<RefreshToken>;
}

#[derive(Clone)]
pub struct PostgresRefreshTokenRepository {
    pool: PgPool,
}

impl PostgresRefreshTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for PostgresRefreshTokenRepository {
    async fn find_by_id(&self, id: Uuid) -> DatabaseResult<Option<RefreshToken>> {
        todo!()
    }

    async fn create(&self) -> DatabaseResult<RefreshToken> {
        todo!()
    }

    async fn update(&self, id: Uuid) -> DatabaseResult<RefreshToken> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> DatabaseResult<RefreshToken> {
        todo!()
    }
}
