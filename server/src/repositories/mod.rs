use crate::domain::models::User;
use sqlx::PgPool;
use uuid::Uuid;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
pub struct RepositoryError;

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
    pub async fn find_user_by_uuid(&self, uuid: Uuid) -> RepositoryResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| RepositoryError)
    }

    pub async fn find_user_by_name(&self, name: &str) -> RepositoryResult<User> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE name = $1 LIMIT 1;")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| RepositoryError)
    }

    pub async fn create_user(&self, item: User) -> RepositoryResult<()> {
        match sqlx::query("INSERT INTO users (uuid, name, password, created_on, updated_on) VALUES ($1, $2, $3, $4, $5)")
            .bind(item.uuid)
            .bind(&item.name)
            .bind(&item.password_hash)
            .bind(item.created_on)
            .bind(item.updated_on)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(RepositoryError),
        }
    }

    pub async fn delete_user(&self, uuid: Uuid) -> RepositoryResult<()> {
        match sqlx::query("DELETE FROM users WHERE uuid = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(RepositoryError),
        }
    }

    pub async fn update_user_password_hash(
        &self,
        uuid: Uuid,
        password_hash: String,
    ) -> RepositoryResult<()> {
        match sqlx::query("UPDATE users SET password_hash = $2 WHERE uuid = $1")
            .bind(uuid)
            .bind(password_hash)
            .execute(&self.pool)
            .await
        {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
