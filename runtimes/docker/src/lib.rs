use async_trait::async_trait;
use bollard::{
    container::{ListContainersOptions, StartContainerOptions, StopContainerOptions},
    image::ListImagesOptions,
    Docker,
};
use container_core::{
    container::{Container, ContainerProvider},
    image::{Image, ImageProvider},
    CorePlugin, Error,
};
use futures_util::Future;
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
    async fn list_images(&self) -> Result<Vec<Image>, Error> {
        let options = ListImagesOptions::<String>::default();

        let list = tokio_run(self.docker.list_images(Some(options)));
        match list {
            Ok(list) => Ok(list
                .into_iter()
                //.inspect(|e| println!("{:?}", e))
                .map(|e| {
                    let mut name = e.repo_tags.into_iter().collect();
                    if name == "<none>:<none>" {
                        name = e.id.clone();
                    }
                    Image::new(e.id, name, e.size as usize)
                })
                .collect()),
            Err(err) => Err(Error(1, err.to_string())),
        }
    }
}

fn is_container_running(state: Option<String>) -> bool {
    match state {
        Some(state) => state == "running",
        None => false,
    }
}

fn tokio_run<F: Future>(future: F) -> F::Output {
    let rt = TOKIO_RT.get().expect("initialize is not called");
    rt.block_on(future)
}

#[async_trait]
impl ContainerProvider for DockerClient {
    async fn list_containers(&self) -> Result<Vec<Container>, Error> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let list = tokio_run(self.docker.list_containers(options));

        match list {
            Ok(list) => Ok(list
                .into_iter()
                //.inspect(|e| println!("{:?}", e))
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
                        let mut n: String = names.into_iter().collect();
                        n.remove(0);
                        n
                    } else {
                        "".to_string()
                    };
                    let running = is_container_running(e.state);
                    Container::new(id, name, image, running, e.status.unwrap_or_default())
                })
                .collect()),
            Err(err) => Err(Error(1, err.to_string())),
        }
    }

    async fn start_container(&self, id: String) -> Result<(), Error> {
        println!("start container {id}");
        let res = tokio_run(
            self.docker
                .start_container(&id, None::<StartContainerOptions<String>>),
        );

        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error(2, err.to_string())),
        }
    }

    async fn stop_container(&self, id: String) -> Result<(), Error> {
        println!("stop container {id}");
        let res = tokio_run(
            self.docker
                .stop_container(&id, None::<StopContainerOptions>),
        );
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error(2, err.to_string())),
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
