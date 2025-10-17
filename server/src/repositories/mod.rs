use uuid::Uuid;

use crate::error::RepositoryResult;

#[cfg(test)]
pub mod mocks;

pub mod refresh_token;
pub mod user;

pub trait Repository<Model> {
    async fn find_by_id(id: Uuid) -> RepositoryResult<Option<Model>>;
}
