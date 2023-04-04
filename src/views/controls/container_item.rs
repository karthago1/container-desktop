use iced::widget::{button, image, text};
use iced_native::widget::container;

#[derive(Debug, Clone)]
pub struct ContainerItem {
    name: String,
    status: Status,
    image: String,
}

#[derive(Debug, Clone)]
pub enum Status {
    Exited,
    Running,
}

impl ContainerItem {
    pub fn new(name: String, status: Status, image: String) -> Self {
        Self {
            name,
            status,
            image,
        }
    }

    pub fn get<'a, Message: 'a>(
        &self,
        index: usize,
        height: f32,
    ) -> impl Into<iced::Element<'a, Message, iced::Renderer>> {
        match index {
            0 => container(image(format!(
                "{}/icons/container.png",
                env!("CARGO_MANIFEST_DIR")
            )))
            .style(if let Status::Running = self.status {
                iced::theme::Container::Box
            } else {
                iced::theme::Container::Transparent
            }),
            1 => container(text(&self.name).size(height * 0.5)).padding(4),
            2 => container(text(&self.image).size(height * 0.5)),
            /*3 => container(button(image(format!(
                "{}/icons/play.png",
                env!("CARGO_MANIFEST_DIR")
            )))),*/
            _ => container(text("Unsupported")),
        }
        .height(height)
        .center_y()
    }
}
