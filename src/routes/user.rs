use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{
    Extension, Json, Router,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::{self, DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgPool};
use tower::ServiceBuilder;
use uuid::Uuid;

use super::auth;

#[derive(Serialize, Deserialize, FromRow)]
pub struct GetUser {
    pub uuid: Uuid,
    pub name: String,
    pub password: String,
    pub created_on: DateTime<Local>,
    pub updated_on: DateTime<Local>,
}

#[derive(Serialize, Deserialize)]
pub struct PostUser {
    pub name: String,
    pub password: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/user", get(get_user).post(post_user))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(auth::authorize)))
}

#[axum::debug_handler]
pub async fn get_user(
    Extension(pool): Extension<PgPool>,
    Extension(user): Extension<Uuid>,
) -> Response {
    match sqlx::query_as::<_, GetUser>("SELECT * FROM users WHERE uuid = $1")
        .bind(user)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn post_user(Extension(pool): Extension<PgPool>, Json(user): Json<PostUser>) -> Response {
    let now = chrono::offset::Local::now();

    match sqlx::query("INSERT INTO users (uuid, name, password, created_on, updated_on) VALUES ($1, $2, $3, $4, $5)")
        .bind(Uuid::now_v7())
        .bind(&user.name)
        .bind(&user.password)
        .bind(now)
        .bind(now)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(sqlx::Error::Database(error)) => {
    
            if error.code().unwrap() == "23505" {
                StatusCode::CONFLICT.into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub fn hash_password<S: Into<String>>(password: S) -> String {
    let argon2 = Argon2::default();
    let salt_string = SaltString::encode_b64(Uuid::now_v7().as_bytes()).unwrap();
    let password_hash = argon2.hash_password(password.into().as_bytes(), &salt_string).unwrap();
    password_hash.to_string()
}
