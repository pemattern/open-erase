pub mod hashing;
pub mod token;

use std::sync::Arc;

use uuid::Uuid;

use crate::{
    ServiceResult,
    error::ServiceError,
    models::User,
    repositories::{DatabaseRepository, user::DatabaseUserRepository},
};

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
    pub async fn find_user_by_id(&self, id: Uuid) -> ServiceResult<Option<User>> {
        Ok(self.user().find_by_id(id).await?)
    }

    pub async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<User>> {
        Ok(self.user().find_by_email(email).await?)
    }

    pub async fn create_user(&self, email: String, password_hash: String) -> ServiceResult<User> {
        Ok(self.user().create(email, password_hash).await?)
    }

    pub async fn delete_user(&self, uuid: Uuid) -> ServiceResult<User> {
        Ok(self.user().delete(uuid).await?)
    }
}
