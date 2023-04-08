use image::ImageProvider;

pub mod image;

#[derive(Debug)]
pub struct CorePlugin {
    pub image_provider: Box::<dyn ImageProvider>,
}
