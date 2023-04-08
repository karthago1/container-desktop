use async_trait::async_trait;
use bollard::{container::ListContainersOptions, image::ListImagesOptions, Docker};
use container_core::{
    container::{Container, ContainerProvider},
    image::{Image, ImageProvider},
    CorePlugin,
};
use tokio::{runtime::Runtime, sync::OnceCell};

#[derive(Debug)]
pub struct DockerClient {
    docker: Docker,
}

static TOKIO_RT: OnceCell<Runtime> = OnceCell::const_new();

impl Default for DockerClient {
    fn default() -> Self {
        Self {
            docker: Docker::connect_with_socket_defaults().unwrap(),
        }
    }
}

impl CorePlugin for DockerClient {}

#[async_trait]
impl ImageProvider for DockerClient {
    async fn list_images(&self) -> Option<Vec<Image>> {
        let rt = TOKIO_RT.get().expect("initialize is not called");

        let options = ListImagesOptions::<String>::default();

        let list = rt.block_on(self.docker.list_images(Some(options)));

        match list {
            Ok(list) => Some(
                list.into_iter()
                    //.inspect(|e| println!("{:?}", e))
                    .map(|e| {
                        let name = e.repo_tags.into_iter().collect();
                        Image::new(e.id, name, e.size as usize)
                    })
                    .collect(),
            ),
            Err(_) => None,
        }
    }
}

#[async_trait]
impl ContainerProvider for DockerClient {
    async fn list_containers(&self) -> Option<Vec<Container>> {
        let rt = TOKIO_RT.get().expect("initialize is not called");

        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let list = rt.block_on(self.docker.list_containers(options));

        match list {
            Ok(list) => Some(
                list.into_iter()
                    .inspect(|e| println!("{:?}", e))
                    .map(|e| {
                        let image = if let Some(img) = e.image {
                            img
                        } else if let Some(id) = e.image_id {
                            id
                        } else {
                            "".to_string()
                        };
                        let id = e.id.unwrap();
                        let name = if let Some(names) = e.names {
                            names.into_iter().collect()
                        } else {
                            "No Name".to_string()
                        };
                        let running = true;
                        Container::new(id, name, image, running)
                    })
                    .collect(),
            ),
            Err(_) => None,
        }
    }
}

#[no_mangle]
pub fn initialize() -> Box<dyn CorePlugin> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.enter();
    TOKIO_RT.set(rt).unwrap();

    Box::<DockerClient>::default()
}
