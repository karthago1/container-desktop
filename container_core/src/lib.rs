use container::ContainerProvider;
use image::ImageProvider;

pub mod container;
pub mod image;

pub trait CorePlugin: ImageProvider + ContainerProvider {
    fn is_clone_supported(&self) -> bool;
    fn get_name(&self) -> String;
    fn is_image_provide_supported(&self) -> bool;
}
