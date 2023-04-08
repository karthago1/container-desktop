use std::fmt::Debug;

use async_trait::async_trait;

pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub running: bool,
}

impl Container {
    pub fn new(id: String, name: String, image: String, running: bool) -> Self {
        Self { id, name, image, running }
    }
}

#[async_trait]
pub trait ContainerProvider: Send + Sync + Debug {
    async fn list_containers(&self) -> Option<Vec<Container>>;
}
