use container::ContainerProvider;
use image::ImageProvider;

pub mod container;
pub mod image;

#[derive(Debug)]
pub struct Error(pub i32, pub String);

pub trait CorePlugin: ImageProvider + ContainerProvider {}
