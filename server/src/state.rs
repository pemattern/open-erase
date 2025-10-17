use std::{env, sync::Arc};

use sqlx::postgres::PgPoolOptions;

use crate::{
    repositories::{refresh_token::PostgresRefreshTokenRepository, user::PostgresUserRepository},
    services::{auth::AuthService, user::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    pub user_service: UserService,
}

impl AppState {
    pub async fn postgres() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = db_url_from_envs();
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let refresh_token_repository = Arc::new(PostgresRefreshTokenRepository::new(pool.clone()));
        let auth_service =
            AuthService::new(user_repository.clone(), refresh_token_repository.clone());
        let user_service = UserService::new(user_repository.clone());
        Ok(Self {
            auth_service,
            user_service,
        })
    }
}

fn db_url_from_envs() -> String {
    let username = env::var("POSTGRES_USER").unwrap_or("postgres".into());
    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".into());
    let host = env::var("POSTGRES_HOST").unwrap_or("postgres".into());
    let port = env::var("POSTGRES_PORT").unwrap_or("5432".into());
    let db = env::var("POSTGRES_DB").unwrap_or("postgres".into());
    format!("postgres://{username}:{password}@{host}:{port}/{db}")
}

#[cfg(test)]
impl AppState {
    pub fn mock() -> Self {
        let user_repository = Arc::new(crate::repositories::mocks::MockUserRepository::new());
        let refresh_token_repository =
            Arc::new(crate::repositories::mocks::MockRefreshTokenRepository::new());
        let auth_service =
            AuthService::new(user_repository.clone(), refresh_token_repository.clone());
        let user_service = UserService::new(user_repository.clone());
        Self {
            auth_service,
            user_service,
        }
    }
}
