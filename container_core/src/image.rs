use std::fmt::Debug;

use async_trait::async_trait;

use crate::Error;

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
    async fn list_images(&self) -> Result<Vec<Image>, Error>;
}
