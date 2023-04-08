use async_trait::async_trait;
use bollard::{image::ListImagesOptions, Docker};
use container_core::{
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

#[async_trait]
impl ImageProvider for DockerClient {
    async fn list(&self) -> Option<Vec<Image>> {
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

#[no_mangle]
pub fn initialize() -> CorePlugin {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.enter();
    TOKIO_RT.set(rt).unwrap();
    CorePlugin {
        image_provider: Box::<DockerClient>::default(),
    }
}
