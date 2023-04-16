use anyhow::Result;
use std::fmt::Debug;

use async_trait::async_trait;

#[derive(Debug)]
pub struct Image {
    pub id: String,
    pub name: String,
    pub size: usize,
}

impl Image {
    pub fn new(id: String, name: String, size: usize) -> Self {
        Self { id, name, size }
    }
}

#[async_trait]
pub trait ImageProvider: Send + Sync + Debug {
    async fn list_images(&self) -> Result<Vec<Image>>;
    async fn export_image(&self, id: String, path: String) -> Result<()>;
    async fn delete_image(&self, id: String) -> Result<()>;
}
