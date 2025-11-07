use async_trait::async_trait;
use std::fs;

use crate::{error::RepositoryResult, models::Image};

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn get_all(&self) -> RepositoryResult<Vec<Image>>;
}

pub struct FsImageRepository;

#[async_trait]
impl ImageRepository for FsImageRepository {
    async fn get_all(&self) -> RepositoryResult<Vec<Image>> {
        let mut images = Vec::new();
        for entry in fs::read_dir("/dist/iso")? {
            let entry = entry?;
            let path = entry.path();
            images.push(Image {
                name: String::from(path.to_str().unwrap()),
            });
        }
        Ok(images)
    }
}
