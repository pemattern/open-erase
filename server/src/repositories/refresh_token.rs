use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::RepositoryResult, models::RefreshToken};

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: Uuid) -> RepositoryResult<Vec<RefreshToken>>;
    async fn create(
        &self,
        user_id: Uuid,
        refresh_token_hash: String,
    ) -> RepositoryResult<RefreshToken>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<RefreshToken>;
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
    async fn find_by_user_id(&self, id: Uuid) -> RepositoryResult<Vec<RefreshToken>> {
        let refresh_token =
            sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens WHERE id = $1;")
                .bind(id)
                .fetch_all(&self.pool)
                .await?;
        Ok(refresh_token)
    }

    async fn create(
        &self,
        user_id: Uuid,
        refresh_token_hash: String,
    ) -> RepositoryResult<RefreshToken> {
        let user = sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_tokens (user_id, token_hash) VALUES ($1, $2) RETURNING *;",
        )
        .bind(user_id)
        .bind(&refresh_token_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<RefreshToken> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            "DELETE FROM refresh_tokens WHERE id = $1 RETURNING *;",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        Ok(refresh_token)
    }
}
