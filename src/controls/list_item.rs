use iced::{
    alignment::Vertical,
    theme,
    widget::{button, text},
};

use super::ui::*;

#[derive(Debug, Clone)]
pub struct ListItem<'a>(pub Vec<ListCell<'a>>);

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

#[derive(Debug, Clone)]
pub enum ListCell<'a> {
    IconStatus(&'a str),
    TextButton(&'a str),
    IconButton(&'a str),
    IconToggleButton(&'a str, &'a str),
}

impl<'a> ListItem<'a> {
    pub fn columns(&self) -> usize {
        self.0.len()
    }

    pub fn get(
        &self,
        index: usize,
        height: f32,
        status: Status,
    ) -> iced::Element<ContainerItemMsg> {
        let e = &self.0[index];

        match e {
            ListCell::IconStatus(icon) => icon_status(&icon, status.is_running())
                .height(height)
                .into(),

            ListCell::TextButton(title) => button(
                text(&title)
                    .size(height * 0.5)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Button::Text)
            .height(height)
            .on_press(ContainerItemMsg(index))
            .into(),

            ListCell::IconButton(icon) => icon_button(&icon)
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),

            ListCell::IconToggleButton(active, disabled) => icon_button(&active)
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),
        }
    }
}
