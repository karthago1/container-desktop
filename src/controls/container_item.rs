use iced::{
    alignment::Vertical,
    theme,
    widget::{button, text},
};

use super::ui::*;

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

impl Status {
    pub fn is_running(&self) -> bool {
        matches!(*self, Status::Running)
    }
}

#[derive(Debug, Clone)]
pub struct ContainerItemMsg(pub usize);

impl ContainerItem {
    pub fn new(name: String, status: Status, image: String) -> Self {
        Self {
            name,
            status,
            image,
        }
    }

    pub fn columns() -> usize {
        5
    }
    pub fn get(&self, index: usize, height: f32) -> iced::Element<ContainerItemMsg> {
        match index {
            0 => icon_status("container.png", self.status.is_running())
                .height(height)
                .into(),
            1 => button(
                text(&self.name)
                    .size(height * 0.5)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Button::Text)
            .height(height)
            .on_press(ContainerItemMsg(index))
            .into(),
            2 => button(
                text(&self.image)
                    .size(height * 0.5)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Button::Text)
            .height(height)
            .on_press(ContainerItemMsg(index))
            .into(),
            3 => icon_button("play.png")
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),
            4 => icon_button("delete.png")
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),
            _ => text("Unsupported").height(height).into(),
        }
    }
}
