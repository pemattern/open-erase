use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::models::User, repositories::PostgresRepository, utils::password_hasher::hash_password,
};

pub type ServiceResult<T> = Result<T, ServiceError>;
pub enum ServiceError {
    RowNotFound,
    Internal,
}

#[derive(Clone)]
pub struct PostgresService {
    pub repository: PostgresRepository,
}

impl PostgresService {
    pub fn new(pool: &PgPool) -> Self {
        let repository = PostgresRepository::new(pool.clone());
        Self { repository }
    }

    pub async fn find_user_by_uuid(&self, uuid: Uuid) -> ServiceResult<User> {
        self.repository
            .find_user_by_uuid(uuid)
            .await
            .map_err(|_| ServiceError::RowNotFound)
    }

    pub async fn find_user_by_name(&self, name: &str) -> ServiceResult<User> {
        self.repository
            .find_user_by_name(name)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn create_user(&self, name: String, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(password);
        let user = User::new(name, password_hash);
        self.repository
            .create_user(user)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<()> {
        self.repository
            .delete_user(uuid)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn update_user_password(&self, uuid: Uuid, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(password);
        self.repository
            .update_user_password_hash(uuid, password_hash)
            .await
            .map_err(|_| ServiceError::Internal)
    }
}
