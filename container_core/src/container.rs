use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub running: bool,
    pub status: String,
}

impl Container {
    pub fn new(id: String, name: String, image: String, running: bool, status: String) -> Self {
        Self {
            id,
            name,
            image,
            running,
            status,
        }
    }
}

#[async_trait]
pub trait ContainerProvider: Send + Sync + Debug {
    async fn list_containers(&self) -> Result<Vec<Container>>;
    async fn start_container(&self, id: String) -> Result<()>;
    async fn stop_container(&self, id: String) -> Result<()>;
    //async fn container_info(&self, id: String) -> Result<(), Error>;
}
