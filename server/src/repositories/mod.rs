pub mod user;

use crate::repositories::user::{DatabaseUserRepository, PostgresUserRepository};
use sqlx::PgPool;
use thiserror::Error;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("an unexpected postgres error occured")]
    Postgres(#[from] sqlx::Error),
}

pub trait DatabaseRepository: Send + Sync {
    fn user(&self) -> &dyn DatabaseUserRepository;
}

#[derive(Clone)]
pub struct PostgresRepository {
    user: PostgresUserRepository,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        let user = PostgresUserRepository::new(pool.clone());
        Self { user }
    }
}

impl DatabaseRepository for PostgresRepository {
    fn user(&self) -> &dyn DatabaseUserRepository {
        &self.user
    }
}
