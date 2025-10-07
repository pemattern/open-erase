use argon2::password_hash;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    ApiResult,
    auth::password::hash_password,
    error::ErrorResponse,
    repositories::PostgresRepository,
    schemas::user::{UserPasswordHash, UserResponse},
};

pub type ServiceResult<T> = Result<T, ServiceError>;
pub enum ServiceError {
    Database(sqlx::Error),
    Hash(password_hash::Error),
    Auth,
}

impl ServiceError {
    pub fn into_api_response(self) -> ApiResult {
        ErrorResponse::internal_server_error()
    }
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

    pub async fn find_user_by_uuid(&self, uuid: Uuid) -> ServiceResult<UserResponse> {
        self.repository
            .find_user_by_uuid(uuid)
            .await
            .map(UserResponse::from)
            .map_err(ServiceError::Database)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<UserResponse> {
        self.repository
            .find_user_by_email(email)
            .await
            .map(UserResponse::from)
            .map_err(ServiceError::Database)
    }

    pub async fn find_user_password_hash_by_email(
        &self,
        email: &str,
    ) -> ServiceResult<UserPasswordHash> {
        self.repository
            .find_user_by_email(email)
            .await
            .map(UserPasswordHash::from)
            .map_err(ServiceError::Database)
    }

    pub async fn create_user(&self, email: String, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(&password)?;
        self.repository
            .create_user(email, password_hash)
            .await
            .map_err(ServiceError::Database)
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<()> {
        self.repository
            .delete_user(uuid)
            .await
            .map_err(ServiceError::Database)
    }

    pub async fn update_user_password(&self, uuid: Uuid, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(&password)?;
        self.repository
            .update_user_password_hash(uuid, password_hash)
            .await
            .map_err(ServiceError::Database)
    }
}
