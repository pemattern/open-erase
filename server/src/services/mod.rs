use sqlx::PgPool;
use users::UserService;

use crate::repositories::users::PostgresUserRepository;

pub mod users;

pub type ServiceResult<T> = Result<T, ServiceError>;
pub enum ServiceError {
    RowNotFound,
    Internal,
}

#[derive(Clone)]
pub struct PostgresService {
    pub users: UserService<PostgresUserRepository>,
}

impl PostgresService {
    pub fn new(pool: &PgPool) -> Self {
        let users = UserService::new(PostgresUserRepository::new(pool.clone()));
        Self { users }
    }
}
