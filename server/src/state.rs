use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::{
    config::Config,
    repositories::{DatabaseRepository, PostgresRepository},
    services::{database::DatabaseService, hashing::HashingService, token::TokenService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub database_service: DatabaseService,
    pub hashing_service: HashingService,
    pub token_service: TokenService,
}

impl AppState {
    pub async fn postgres() -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = db_url_from_envs();
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let repo = PostgresRepository::new(pool);
        Ok(Self::init(repo))
    }

    fn init(repo: impl DatabaseRepository + 'static) -> Self {
        let config = Config::load();
        let database_service = DatabaseService::new(repo);
        let hashing_service = HashingService;
        let token_service = TokenService;
        Self {
            config,
            database_service,
            hashing_service,
            token_service,
        }
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
        let repo = crate::repositories::mocks::MockRepository::new();
        Self::init(repo)
    }
}
