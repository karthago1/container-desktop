use std::fmt::Debug;

use async_trait::async_trait;

use crate::Error;

pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub running: bool,
}

impl Container {
    pub fn new(id: String, name: String, image: String, running: bool) -> Self {
        Self {
            id,
            name,
            image,
            running,
        }
    }
}

#[async_trait]
pub trait ContainerProvider: Send + Sync + Debug {
    async fn list_containers(&self) -> Result<Vec<Container>, Error>;
    async fn start_container(&self, id: String) -> Result<(), Error>;
    async fn stop_container(&self, id: String) -> Result<(), Error>;
}
