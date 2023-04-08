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
pub struct ListItemMsg(pub usize);

#[derive(Debug, Clone)]
pub enum ListCell {
    IconStatus(&'static str, bool),
    TextButton(String),
    IconButton(&'static str),
    IconToggleButton(&'static str, &'static str, bool),
}

impl ListItem {
    pub fn get(
        &self,
        index: usize,
        width: iced::Length,
        height: f32,
    ) -> iced::Element<ListItemMsg> {
        let e = &self.0[index];

        match e {
            ListCell::IconStatus(icon, status) => icon_status(icon, *status)
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
            .on_press(ListItemMsg(index))
            .into(),

            ListCell::IconButton(icon) => icon_button(icon)
                .width(width)
                .height(height)
                .on_press(ListItemMsg(index))
                .into(),

            ListCell::IconToggleButton(active, disabled, status) => {
                icon_button(if *status { disabled } else { active })
                    .width(width)
                    .height(height)
                    .on_press(ListItemMsg(index))
                    .into()
            }
        }
    }
}
