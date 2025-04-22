use chrono::Local;
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::user::hash_password;

pub async fn initialize_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;

    let uuid = Uuid::nil();
    let name = "system";
    let password = hash_password("secret", "saltymcsaltster");
    let now = Local::now();

    sqlx::query(
        "INSERT INTO users (uuid, name, password_hash, created_at, modified_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT DO NOTHING",
    )
    .bind(uuid)
    .bind(name)
    .bind(password)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}
