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
pub enum ListItemMsg {
    Clicked(usize, u32),
    ChangeCell(usize, ListCell),
    TextChanged(usize, String),
}

#[derive(Debug, Clone)]
pub enum ListCell {
    IconStatus(usize, bool),
    TextButton(String, u32),
    IconButton(usize, u32),
}

impl ListItem {
    pub fn get_view(
        &self,
        index: usize,
        width: iced::Length,
        height: f32,
    ) -> iced::Element<ListItemMsg> {
        let e = &self.0[index];

        match e {
            ListCell::IconStatus(icon, status) => icon_status(*icon, *status)
                .width(width)
                .height(height)
                .into(),

            ListCell::TextButton(title, action) => button(
                text(&title)
                    .size(style::FONT_SIZE_SMALL)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Button::Text)
            .width(width)
            .height(height)
            .on_press(ListItemMsg::Clicked(index, *action))
            .into(),

            ListCell::IconButton(icon, action) => icon_button(*icon)
                .width(width)
                .height(height)
                .on_press(ListItemMsg::Clicked(index, *action))
                .into(),
        }
    }

    pub fn update(&mut self, msg: ListItemMsg) {
        if let ListItemMsg::ChangeCell(index, new_cell) = msg {
            self.0[index] = new_cell;
        }
    }
}
