use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use open_erase_lib::schemas::image::{GetImageResponse, GetImagesResponse};
use serde::{Deserialize, Serialize};

use crate::{models::Image, schemas::json};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerGetImagesResponse(pub GetImagesResponse);

impl IntoResponse for ServerGetImagesResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, json(self.0)).into_response()
    }
}

impl From<Vec<Image>> for ServerGetImagesResponse {
    fn from(value: Vec<Image>) -> Self {
        let images = value
            .into_iter()
            .map(|image| GetImageResponse { name: image.name })
            .collect::<Vec<GetImageResponse>>();
        Self(GetImagesResponse(images))
    }
}
