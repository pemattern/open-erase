use std::sync::Arc;

use uuid::Uuid;

use crate::{error::ServiceResult, models::User, repositories::user::UserRepository};

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

impl UserService {
    pub async fn find_user_by_id(&self, id: Uuid) -> ServiceResult<Option<User>> {
        Ok(self.user_repository.find_by_id(id).await?)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<User>> {
        Ok(self.user_repository.find_by_email(email).await?)
    }

    pub async fn create_user(&self, email: String, password_hash: String) -> ServiceResult<User> {
        Ok(self.user_repository.create(email, password_hash).await?)
    }

    pub async fn update_user(&self, id: Uuid, email: Option<String>) -> ServiceResult<User> {
        Ok(self.user_repository.update(id, email).await?)
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<User> {
        Ok(self.user_repository.delete(uuid).await?)
    }
}
