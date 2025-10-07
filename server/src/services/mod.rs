use argon2::password_hash;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    ApiResult,
    auth::password::hash_password,
    error::ErrorResponse,
    repositories::PostgresRepository,
    schemas::user::{UserPasswordHash, UserResponse},
};

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("an unexpected error occured")]
    Database(#[from] sqlx::Error),
    #[error("unauthorized")]
    Hash(password_hash::Error),
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
        let user = self.repository.find_user_by_uuid(uuid).await?;
        Ok(user.into())
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<UserResponse> {
        let user = self.repository.find_user_by_email(email).await?;
        Ok(user.into())
    }

    pub async fn find_user_password_hash_by_email(
        &self,
        email: &str,
    ) -> ServiceResult<UserPasswordHash> {
        let user = self.repository.find_user_by_email(email).await?;
        Ok(user.into())
    }

    pub async fn create_user(&self, email: String, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(&password)?;
        self.repository.create_user(email, password_hash).await?;
        Ok(())
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<()> {
        self.repository.delete_user(uuid).await?;
        Ok(())
    }

    pub async fn update_user_password(&self, uuid: Uuid, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(&password)?;
        self.repository
            .update_user_password_hash(uuid, password_hash)
            .await?;
        Ok(())
    }
}
