use core::{
    image::{Image, ImageProvider},
    CorePlugin,
};
use std::{thread, time};

#[derive(Default)]
pub struct ImageSimulation;

impl ImageProvider for ImageSimulation {
    fn list(&self) -> Vec<Image> {
        thread::sleep(time::Duration::from_secs(1));
        vec![
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
        ]
    }
}

#[no_mangle]
pub fn initialize() -> CorePlugin {
    CorePlugin {
        image_provider: Box::<ImageSimulation>::default(),
    }
}
