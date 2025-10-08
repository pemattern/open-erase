use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::{
    config::Config,
    services::{PostgresService, hashing::HashingService, token::TokenService},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub postgres_service: PostgresService,
    pub hashing_service: HashingService,
    pub token_service: TokenService,
}

impl AppState {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        tracing_subscriber::fmt().compact().init();
        let db_url = db_url_from_envs()?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        let config = Config::load();
        let postgres_service = PostgresService::new(&pool);
        let hashing_service = HashingService;
        let token_service = TokenService;
        Ok(Self {
            config,
            postgres_service,
            hashing_service,
            token_service,
        })
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
