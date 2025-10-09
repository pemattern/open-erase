mod user;

use crate::repositories::{
    DatabaseRepository, mocks::user::MockUserRepository, user::DatabaseUserRepository,
};

#[derive(Clone)]
pub struct MockRepository {
    user: MockUserRepository,
}

impl MockRepository {
    pub fn new() -> Self {
        let user = MockUserRepository::new();
        Self { user }
    }
}

impl DatabaseRepository for MockRepository {
    fn user(&self) -> &dyn DatabaseUserRepository {
        &self.user
    }
}
