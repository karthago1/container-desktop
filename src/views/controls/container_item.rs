use iced::widget::{row, text};

#[derive(Debug, Clone)]
pub struct ContainerItem {
    name: String,
}

impl ContainerItem {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn view<'a, Message>(&self) -> iced::widget::Row<'a, Message, iced::Renderer> {
        row![text(&self.name)]
    }
}
