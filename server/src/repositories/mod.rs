pub mod users;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
pub struct RepositoryError;
