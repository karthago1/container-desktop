use image::ImageProvider;

pub mod image;

pub struct CorePlugin {
    pub image_provider: Box<dyn ImageProvider>,
}
