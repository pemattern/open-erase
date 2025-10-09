#[cfg(test)]
pub mod mocks;

pub mod user;

use crate::{
    error::DatabaseError,
    repositories::user::{DatabaseUserRepository, PostgresUserRepository},
};

pub type DatabaseResult<T> = Result<T, DatabaseError>;

pub trait DatabaseRepository: Send + Sync {
    fn user(&self) -> &dyn DatabaseUserRepository;
}

#[derive(Clone)]
pub struct PostgresRepository {
    user: PostgresUserRepository,
}

impl PostgresRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let user = PostgresUserRepository::new(pool.clone());
        Self { user }
    }
}

impl DatabaseRepository for PostgresRepository {
    fn user(&self) -> &dyn DatabaseUserRepository {
        &self.user
    }
}
