use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{button, column, container, image, row, text},
    Element, Length,
};

use crate::style;

pub struct MainMenuItem {
    pub text: String,
    pub icon: String,
    pub badge: Option<i32>,
}

#[derive(Default)]
pub struct MainMenu {
    pub selected_index: usize,
    pub items: Vec<MainMenuItem>,
    item_height: f32,
}

#[derive(Debug, Clone)]
pub enum MainMenuMessage {
    SelectedIndex(usize),
    Badge(usize, Option<i32>),
}

impl MainMenuItem {
    pub fn new(text: String, icon: String) -> Self {
        Self {
            text,
            icon,
            badge: None,
        }
    }
}

impl MainMenu {
    pub fn new(items: Vec<MainMenuItem>) -> Self {
        Self {
            selected_index: 0,
            items,
            item_height: 48.,
        }
    }

    pub fn update(&mut self, message: MainMenuMessage) {
        match message {
            MainMenuMessage::SelectedIndex(index) => self.selected_index = index,
            MainMenuMessage::Badge(index, badge) => self.items[index].badge = badge,
        }
    }

    pub fn view(&self) -> Element<MainMenuMessage> {
        column(
            self.items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let mut row = row![
                        container(
                            image(format!(
                                "{}/icons/{}",
                                env!("CARGO_MANIFEST_DIR"),
                                item.icon
                            ))
                            .height(Length::Fill)
                        )
                        .height(Length::Fill)
                        .padding([0, 4]),
                        text(&item.text)
                            .vertical_alignment(Vertical::Center)
                            .width(Length::Fill),
                    ]
                    .align_items(iced::Alignment::Center);

                    if let Some(b) = item.badge {
                        row = row.push(
                            button(
                                text(b.to_string())
                                    .size(14)
                                    .horizontal_alignment(Horizontal::Center)
                                    .style(*style::colors::PRIMARY_TEXT),
                            )
                            .style(theme::Button::Custom(Box::new(style::ButtonChip::new(10.)))),
                        );
                    }

                    button(row)
                        .height(self.item_height)
                        .on_press(MainMenuMessage::SelectedIndex(index))
                        .width(Length::Fill)
                        .style(if index == self.selected_index {
                            theme::Button::Secondary
                        } else {
                            theme::Button::Text
                        })
                        .into()
                })
                .collect(),
        )
        .max_width(220)
        .into()
    }
}
