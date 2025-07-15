use uuid::Uuid;

use crate::utils::password_hasher::hash_password;
use crate::{domain::models::User, repositories::users::UserRepository};

use super::{ServiceError, ServiceResult};

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn find_by_uuid(&self, uuid: Uuid) -> ServiceResult<User> {
        self.repository
            .find_by_uuid(uuid)
            .await
            .map_err(|_| ServiceError::RowNotFound)
    }

    pub async fn find_by_name(&self, name: &str) -> ServiceResult<User> {
        self.repository
            .find_by_name(name)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn create(&self, name: String, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(password);
        let user = User::new(name, password_hash);
        self.repository
            .create(user)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn delete(&self, uuid: Uuid) -> ServiceResult<()> {
        self.repository
            .delete(uuid)
            .await
            .map_err(|_| ServiceError::Internal)
    }

    pub async fn update_password(&self, uuid: Uuid, password: String) -> ServiceResult<()> {
        let password_hash = hash_password(password);
        self.repository
            .update_password_hash(uuid, password_hash)
            .await
            .map_err(|_| ServiceError::Internal)
    }
}
