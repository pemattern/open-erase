use std::sync::Arc;

use crate::{error::ServiceResult, models::Image, repositories::image::ImageRepository};

#[derive(Clone)]
pub struct ImageService {
    pub image_repository: Arc<dyn ImageRepository>,
}

impl ImageService {
    pub fn new(image_repository: Arc<dyn ImageRepository>) -> Self {
        Self { image_repository }
    }
}

impl ImageService {
    pub async fn get_all(&self) -> ServiceResult<Vec<Image>> {
        Ok(self.image_repository.get_all().await?)
    }
}
