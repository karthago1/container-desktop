use iced::{
    alignment::Vertical,
    theme,
    widget::{button, text},
};

use crate::style;

use super::ui::*;

#[derive(Debug, Clone)]
pub struct ListItem(pub Vec<ListCell>);

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
pub enum ListCell {
    IconStatus(&'static str),
    TextButton(&'static str),
    IconButton(&'static str),
    IconToggleButton(&'static str, &'static str),
}

impl ListItem {
    pub fn columns(&self) -> usize {
        self.0.len()
    }

    pub fn get(
        &self,
        index: usize,
        width: iced::Length,
        height: f32,
        status: Status,
    ) -> iced::Element<ContainerItemMsg> {
        let e = &self.0[index];

        match e {
            ListCell::IconStatus(icon) => icon_status(icon, status.is_running())
                .width(width)
                .height(height)
                .into(),

            ListCell::TextButton(title) => button(
                text(&title)
                    .size(style::FONT_SIZE_SMALL)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Button::Text)
            .width(width)
            .height(height)
            .on_press(ContainerItemMsg(index))
            .into(),

            ListCell::IconButton(icon) => icon_button(icon)
                .width(width)
                .height(height)
                .on_press(ContainerItemMsg(index))
                .into(),

            ListCell::IconToggleButton(active, disabled) => icon_button(if status.is_running() {
                disabled
            } else {
                active
            })
            .width(width)
            .height(height)
            .on_press(ContainerItemMsg(index))
            .into(),
        }
    }
}
