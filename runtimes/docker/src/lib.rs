use std::{fs::File, io::Write};

use anyhow::Result;
use async_trait::async_trait;
use bollard::{
    container::{
        ListContainersOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions,
    },
    image::{ListImagesOptions, RemoveImageOptions},
    Docker,
};
use container_core::{
    container::{Container, ContainerProvider},
    image::{Image, ImageProvider},
    CorePlugin,
};
use futures_util::{Future, StreamExt};
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

impl CorePlugin for DockerClient {
    fn is_clone_supported(&self) -> bool {
        false
    }

    fn get_name(&self) -> String {
        "Docker".to_string()
    }

    fn is_image_provide_supported(&self) -> bool {
        true
    }
}

#[async_trait]
impl ImageProvider for DockerClient {
    async fn list_images(&self) -> Result<Vec<Image>> {
        let options = ListImagesOptions::<String>::default();

        let list = tokio_run(self.docker.list_images(Some(options)))?;
        Ok(list
            .into_iter()
            //.inspect(|e| println!("{:?}", e))
            .map(|e| {
                let mut name = e.repo_tags.into_iter().collect();
                if name == "<none>:<none>" {
                    name = "".to_string();
                }
                Image::new(e.id, name, e.size as usize)
            })
            .collect())
    }

    async fn export_image(&self, id: String, path: String) -> Result<()> {
        println!("export image {id} to {path}");
        let mut stream = self.docker.export_image(&id);

        let mut file = File::create(path)?;

        while let Some(b) = tokio_run(stream.next()) {
            match b {
                Ok(b) => file.write_all(&b)?,
                Err(err) => return Err(err.into()),
            }
        }

        Ok(())
    }

    async fn delete_image(&self, id: String) -> Result<()> {
        println!("delete image {id}");
        let options = RemoveImageOptions::default();
        let _list = tokio_run(self.docker.remove_image(&id, Some(options), None))?;
        Ok(())
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
    async fn list_containers(&self) -> Result<Vec<Container>> {
        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let list = tokio_run(self.docker.list_containers(options))?;

        Ok(list
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
            .collect())
    }

    async fn start_container(&self, id: String) -> Result<()> {
        println!("start container {id}");
        tokio_run(
            self.docker
                .start_container(&id, None::<StartContainerOptions<String>>),
        )?;

        Ok(())
    }

    async fn stop_container(&self, id: String) -> Result<()> {
        println!("stop container {id}");
        tokio_run(
            self.docker
                .stop_container(&id, None::<StopContainerOptions>),
        )?;
        Ok(())
    }

    async fn clone_container(&self, _id: String, _new_name: String) -> Result<()> {
        todo!("docker clone container")
    }

    async fn remove_container(&self, id: String) -> Result<()> {
        println!("remove container {id}");
        tokio_run(
            self.docker
                .remove_container(&id, None::<RemoveContainerOptions>),
        )?;

        Ok(())
    }
}

#[no_mangle]
pub fn initialize() -> Box<dyn CorePlugin> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.enter();
    TOKIO_RT.set(rt).unwrap();

    Box::<DockerClient>::default()
}
