use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetImageResponse {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetImagesResponse(pub Vec<GetImageResponse>);
