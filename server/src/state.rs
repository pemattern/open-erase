use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::{
    config::Config,
    repositories::{DatabaseRepository, MockRepository, PostgresRepository},
    services::{DatabaseService, hashing::HashingService, token::TokenService},
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
        let db_url = db_url_from_envs()?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let repo = PostgresRepository::new(pool);
        Ok(Self::init(repo))
    }

    pub fn mock() -> Self {
        let repo = MockRepository::new();
        Self::init(repo)
    }

    fn init(repo: impl DatabaseRepository + 'static) -> Self {
        tracing_subscriber::fmt().compact().init();
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

fn db_url_from_envs() -> Result<String, Box<dyn std::error::Error>> {
    let username = env::var("POSTGRES_USER")?;
    let password = env::var("POSTGRES_PASSWORD")?;
    let host = env::var("POSTGRES_HOST")?;
    let port = env::var("POSTGRES_PORT")?;
    let db = env::var("POSTGRES_DB")?;
    let url = format!("postgres://{username}:{password}@{host}:{port}/{db}");
    Ok(url)
}
