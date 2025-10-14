use crate::{
    handlers::user::{delete_user, get_user, patch_user, post_user},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/{uuid}",
            get(get_user).patch(patch_user).delete(delete_user),
        )
        .route("/", post(post_user))
}
