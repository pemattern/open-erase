use axum::Router;

mod auth;
mod docs;
mod user;

pub fn api_router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/user", user::router())
        .merge(docs::router())
}
