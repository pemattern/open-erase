use crate::models::User;
use sqlx::PgPool;
use uuid::Uuid;

pub type DatabaseResult<T> = Result<T, sqlx::Error>;

#[derive(Clone)]
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PostgresRepository {
    pub async fn find_user_by_uuid(&self, uuid: Uuid) -> DatabaseResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn find_user_by_email(&self, email: &str) -> DatabaseResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_user(&self, email: String, password_hash: String) -> DatabaseResult<()> {
        sqlx::query("INSERT INTO users (email, password_hash) VALUES ($1, $2)")
            .bind(&email)
            .bind(&password_hash)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn delete_user(&self, uuid: Uuid) -> DatabaseResult<()> {
        sqlx::query("DELETE FROM users WHERE uuid = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn update_user_password_hash(
        &self,
        uuid: Uuid,
        password_hash: String,
    ) -> DatabaseResult<()> {
        sqlx::query("UPDATE users SET password_hash = $2 WHERE uuid = $1")
            .bind(uuid)
            .bind(password_hash)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }
}
