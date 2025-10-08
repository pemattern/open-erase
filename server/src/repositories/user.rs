use crate::{models::User, repositories::DatabaseResult};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait DatabaseUserRepository: Send + Sync {
    async fn find_by_uuid(&self, uuid: Uuid) -> DatabaseResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>>;
    async fn create(&self, email: String, password_hash: String) -> DatabaseResult<()>;
    async fn delete(&self, uuid: Uuid) -> DatabaseResult<()>;
    async fn update_password_hash(&self, uuid: Uuid, password_hash: String) -> DatabaseResult<()>;
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
    async fn find_by_uuid(&self, uuid: Uuid) -> DatabaseResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn create(&self, email: String, password_hash: String) -> DatabaseResult<()> {
        sqlx::query("INSERT INTO users (email, password_hash) VALUES ($1, $2)")
            .bind(&email)
            .bind(&password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, uuid: Uuid) -> DatabaseResult<()> {
        sqlx::query("DELETE FROM users WHERE uuid = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_password_hash(&self, uuid: Uuid, password_hash: String) -> DatabaseResult<()> {
        sqlx::query("UPDATE users SET password_hash = $2 WHERE uuid = $1")
            .bind(uuid)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct MockUserRepository;

#[async_trait]
impl DatabaseUserRepository for MockUserRepository {
    async fn find_by_uuid(&self, uuid: Uuid) -> DatabaseResult<Option<User>> {
        let mut user = User::mock();
        user.uuid = uuid;
        Ok(Some(user))
    }

    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>> {
        let mut user = User::mock();
        user.email = email;
        Ok(Some(user))
    }

    async fn create(&self, email: String, password_hash: String) -> DatabaseResult<()> {
        Ok(())
    }

    async fn delete(&self, uuid: Uuid) -> DatabaseResult<()> {
        Ok(())
    }

    async fn update_password_hash(&self, uuid: Uuid, password_hash: String) -> DatabaseResult<()> {
        Ok(())
    }
}
