use crate::{error::RepositoryResult, models::User};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
    async fn create(&self, email: String, password_hash: String) -> RepositoryResult<User>;
    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<User>;
    async fn delete(&self, id: Uuid) -> RepositoryResult<User>;
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
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<User>> {
        let query = "
            SELECT * FROM users
            WHERE id = $1;
        ";
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let query = "
            SELECT * FROM users
            WHERE email = $1;
        ";
        let user = sqlx::query_as::<_, User>(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn create(&self, email: String, password_hash: String) -> RepositoryResult<User> {
        let query = "
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let user = sqlx::query_as::<_, User>(query)
            .bind(&email)
            .bind(&password_hash)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<User> {
        let query = "
            UPDATE users
            SET email = $1
            WHERE id = $2
            RETURNING *;
        ";
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .bind(&email)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<User> {
        let query = "
            DELETE FROM users
            WHERE id = $1
            RETURNING *;  
        ";
        let user = sqlx::query_as::<_, User>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
