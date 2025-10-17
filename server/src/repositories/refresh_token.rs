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
        let query = "
            SELECT * FROM refresh_tokens
            WHERE id = $1;
        ";
        let refresh_token = sqlx::query_as::<_, RefreshToken>(query)
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
        let query = "
            INSERT INTO refresh_tokens (user_id, refresh_token_hash)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let user = sqlx::query_as::<_, RefreshToken>(query)
            .bind(user_id)
            .bind(&refresh_token_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<RefreshToken> {
        let query = "
            DELETE FROM refresh_tokens
            WHERE id = $1
            RETURNING *;  
        ";
        let refresh_token = sqlx::query_as::<_, RefreshToken>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(refresh_token)
    }
}
