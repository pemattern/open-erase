use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    models::User,
    repositories::{DatabaseResult, user::DatabaseUserRepository},
};

#[derive(Clone)]
pub struct MockUserRepository {
    data: Vec<User>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            data: vec![User::mock()],
        }
    }
}

#[async_trait]
impl DatabaseUserRepository for MockUserRepository {
    async fn find_by_uuid(&self, uuid: Uuid) -> DatabaseResult<Option<User>> {
        Ok(self.data.iter().find(|user| user.uuid == uuid).cloned())
    }

    async fn find_by_email(&self, email: &str) -> DatabaseResult<Option<User>> {
        Ok(self.data.iter().find(|user| user.email == email).cloned())
    }

    async fn create(&self, _email: String, _password_hash: String) -> DatabaseResult<()> {
        Ok(())
    }

    async fn delete(&self, _uuid: Uuid) -> DatabaseResult<()> {
        Ok(())
    }
}
