use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration};
use uuid::Uuid;

use crate::{
    error::{RepositoryError, RepositoryResult},
    models::RefreshToken,
    repositories::refresh_token::RefreshTokenRepository,
};

impl RefreshToken {
    pub fn mock() -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            parent_id: None,
            // password123
            opaque_token_hash: String::from(
                "$argon2id$v=19$m=16,t=2,p=1$NjFWcEMwUEQ0dmZXcDMwSg$TfJtuSrudRp6hhV2mFSt3g",
            ),
            is_used: false,
            expires_at: DateTime::default() + Duration::days(60),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

#[derive(Clone)]
pub struct MockRefreshTokenRepository {
    data: Arc<Mutex<Vec<RefreshToken>>>,
}

impl MockRefreshTokenRepository {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![RefreshToken::mock()])),
        }
    }
}

#[async_trait]
impl RefreshTokenRepository for MockRefreshTokenRepository {
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Option<RefreshToken>> {
        Ok(self
            .data
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .find(|refresh_token| refresh_token.id == id && refresh_token.is_valid()))
    }

    async fn create(
        &self,
        user_id: Uuid,
        parent_id: Option<Uuid>,
        opaque_token_hash: String,
    ) -> RepositoryResult<RefreshToken> {
        let mut refresh_token = RefreshToken::mock();
        refresh_token.user_id = user_id;
        refresh_token.parent_id = parent_id;
        refresh_token.opaque_token_hash = opaque_token_hash;
        let mut data = self.data.lock().unwrap();
        data.push(refresh_token.clone());
        Ok(refresh_token)
    }

    async fn mark_as_used(&self, id: Uuid) -> RepositoryResult<RefreshToken> {
        if let Some(mut refresh_token) = self
            .data
            .lock()
            .unwrap()
            .iter_mut()
            .find(|refresh_token| refresh_token.id == id)
            .cloned()
        {
            refresh_token.is_used = true;
            return Ok(refresh_token);
        }
        Err(RepositoryError::Test)
    }

    async fn delete(&self, id: Uuid) -> RepositoryResult<RefreshToken> {
        let mut data = self.data.lock().unwrap();
        let user = data
            .extract_if(.., |user| user.id == id)
            .collect::<Vec<RefreshToken>>()
            .first()
            .cloned();
        user.ok_or(RepositoryError::Test)
    }
}
