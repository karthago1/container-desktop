use container::ContainerProvider;
use image::ImageProvider;

pub mod container;
pub mod image;

pub trait CorePlugin: ImageProvider + ContainerProvider {}
