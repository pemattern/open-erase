use axum::{Router, routing::post};

use crate::{
    handlers::auth::{login, refresh},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
}
