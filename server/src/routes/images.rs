use axum::{Router, routing::get};

use crate::{handlers::images, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(images::images))
}
