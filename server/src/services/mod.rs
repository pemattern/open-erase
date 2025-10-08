pub mod hashing;
pub mod token;

use std::sync::Arc;

use argon2::password_hash;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    repositories::{DatabaseError, DatabaseRepository, user::DatabaseUserRepository},
    schemas::user::{UserPasswordHash, UserResponse},
};

pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("an unexpected error occured")]
    Database(#[from] DatabaseError),
    #[error("unauthorized")]
    Hash(password_hash::Error),
    #[error("an unexpected error occured")]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("an unexpected error occured")]
    Uuid(#[from] uuid::Error),
}

#[derive(Clone)]
pub struct DatabaseService {
    repo: Arc<dyn DatabaseRepository>,
}

impl DatabaseService {
    pub fn new(repo: impl DatabaseRepository + 'static) -> Self {
        let repo = Arc::new(repo);
        Self { repo }
    }

    fn user(&self) -> &dyn DatabaseUserRepository {
        self.repo.user()
    }
}

impl DatabaseService {
    pub async fn find_user_by_uuid(&self, uuid: Uuid) -> ServiceResult<Option<UserResponse>> {
        let user = self.user().find_by_uuid(uuid).await?;
        Ok(user.map(UserResponse::from))
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<UserResponse>> {
        let user = self.user().find_by_email(email).await?;
        Ok(user.map(UserResponse::from))
    }

    pub async fn find_user_password_hash_by_email(
        &self,
        email: &str,
    ) -> ServiceResult<Option<UserPasswordHash>> {
        let user = self.user().find_by_email(email).await?;
        Ok(user.map(UserPasswordHash::from))
    }

    pub async fn create_user(&self, email: String, password_hash: String) -> ServiceResult<()> {
        self.user().create(email, password_hash).await?;
        Ok(())
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<()> {
        self.user().delete(uuid).await?;
        Ok(())
    }

    pub async fn update_user_password(
        &self,
        uuid: Uuid,
        password_hash: String,
    ) -> ServiceResult<()> {
        self.user()
            .update_password_hash(uuid, password_hash)
            .await?;
        Ok(())
    }
}
