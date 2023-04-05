use iced::{
    alignment::Vertical,
    theme,
    widget::{button, text},
    Length,
};

use crate::ui;

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
        match *self {
            Status::Running => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerItemMsg(usize);

impl ContainerItem {
    pub fn new(name: String, status: Status, image: String) -> Self {
        Self {
            name,
            status,
            image,
        }
    }

    pub fn columns() -> usize {
        return 5;
    }
    pub fn get(&self, index: usize, height: f32) -> iced::Element<ContainerItemMsg> {
        match index {
            0 => ui::icon_status("container.png", self.status.is_running())
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
            3 => ui::icon_button("play.png")
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),
            4 => ui::icon_button("delete.png")
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),
            _ => text("Unsupported").height(height).into(),
        }
    }
}
