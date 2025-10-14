use crate::{error::DatabaseResult, models::User};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait DatabaseUserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> DatabaseResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>>;
    async fn create(&self, email: String, password_hash: String) -> DatabaseResult<User>;
    async fn delete(&self, uuid: Uuid) -> DatabaseResult<User>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseUserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> DatabaseResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1;")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1;")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn create(&self, email: String, password_hash: String) -> DatabaseResult<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *;",
        )
        .bind(&email)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> DatabaseResult<User> {
        let user = sqlx::query_as::<_, User>("DELETE FROM users WHERE id = $1 RETURNING *;")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
