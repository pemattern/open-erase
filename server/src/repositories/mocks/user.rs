use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::DateTime;
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::User,
    repositories::user::UserRepository,
};

impl User {
    pub fn mock() -> Self {
        Self {
            id: Uuid::default(),
            email: String::from("test@mail.com"),
            password_hash: String::from(
                "$argon2id$v=19$m=16,t=2,p=1$NjFWcEMwUEQ0dmZXcDMwSg$TfJtuSrudRp6hhV2mFSt3g",
            ),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

#[derive(Clone)]
pub struct MockUserRepository {
    data: Arc<Mutex<Vec<User>>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![User::mock()])),
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, uuid: Uuid) -> RepositoryResult<Option<User>> {
        Ok(self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id == uuid)
            .cloned())
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        Ok(self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.email == email)
            .cloned())
    }

    async fn create(&self, email: String, password_hash: String) -> RepositoryResult<User> {
        let mut user = User::mock();
        user.email = email;
        user.password_hash = password_hash;
        let mut data = self.data.lock().unwrap();
        data.push(user.clone());
        Ok(user)
    }

    async fn update(&self, id: Uuid, email: Option<String>) -> RepositoryResult<User> {
        if let Some(email) = email {
            self.data
                .lock()
                .unwrap()
                .iter_mut()
                .filter(|user| user.id == id)
                .for_each(|user| user.email = email.clone());
        };
        let user = self
            .data
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id == id)
            .cloned();
        user.ok_or(RepositoryError::Test)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<User> {
        let mut data = self.data.lock().unwrap();
        let user = data
            .extract_if(.., |user| user.id == id)
            .collect::<Vec<User>>()
            .first()
            .cloned();
        user.ok_or(RepositoryError::Test)
    }
}
