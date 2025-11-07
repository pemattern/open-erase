use axum::extract::State;

use crate::{error::AppResult, schemas::image::ServerGetImagesResponse, state::AppState};

#[axum::debug_handler]
#[utoipa::path(get, path = "/images")]
pub async fn images(State(state): State<AppState>) -> AppResult<ServerGetImagesResponse> {
    Ok(state.image_service.get_all().await?.into())
}
