use anyhow::Result;
use async_trait::async_trait;
use container_core::{
    container::{Container, ContainerProvider},
    image::{Image, ImageProvider},
    CorePlugin,
};
use std::{future, thread, time};

#[derive(Default, Debug)]
pub struct Simulation;

impl CorePlugin for Simulation {
    fn is_clone_supported(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        "Simulator".to_string()
    }
}

#[async_trait]
impl ImageProvider for Simulation {
    async fn list_images(&self) -> Result<Vec<Image>> {
        thread::sleep(time::Duration::from_secs(1));
        future::ready(Ok::<Vec<Image>, anyhow::Error>(vec![
            Image::new(
                "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Ubuntu".to_string(),
                713 * 1024 * 1024,
            ),
            Image::new(
                "sha256:1119d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Debian".to_string(),
                613 * 1024 * 1024,
            ),
            Image::new(
                "sha256:2229d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "".to_string(),
                700 * 1024 * 1024,
            ),
            Image::new(
                "sha256:3339d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Alpine".to_string(),
                19 * 1024 * 1024,
            ),
            Image::new(
                "sha256:4449d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Alpine-lts".to_string(),
                61 * 1024 * 1024,
            ),
        ]))
        .await
    }
}

#[async_trait]
impl ContainerProvider for Simulation {
    async fn list_containers(&self) -> Result<Vec<Container>> {
        future::ready(Ok(vec![
            Container::new(
                "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Ubuntu Container".to_string(),
                "Ubuntu".to_string(),
                true,
                "Up 29 Seconds".to_string(),
            ),
            Container::new(
                "sha256:1119d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Debian Container".to_string(),
                "Debian".to_string(),
                false,
                "Exited (137) 2 months ago".to_string(),
            ),
            Container::new(
                "sha256:2229d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                    .to_string(),
                "Custom Container".to_string(),
                "sha256:5654657c21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39123"
                    .to_string(),
                true,
                "Up 59 Seconds".to_string(),
            ),
        ]))
        .await
    }

    async fn start_container(&self, _id: String) -> Result<()> {
        Ok(())
    }
    async fn stop_container(&self, _id: String) -> Result<()> {
        Ok(())
    }
    async fn clone_container(&self, _id: String, _new_name: String) -> Result<()> {
        Ok(())
    }
    async fn remove_container(&self, _id: String) -> Result<()> {
        Ok(())
    }
}

#[no_mangle]
pub fn initialize() -> Box<dyn CorePlugin> {
    Box::<Simulation>::default()
}
