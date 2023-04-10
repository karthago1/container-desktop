use iced::{
    theme,
    widget::{container, scrollable},
    Length,
};

use crate::{
    controls::list_item::{ListItem, ListItemMsg},
    style,
};

pub struct ListView {
    state: State,
    columns_width: Vec<iced::Length>,
}

#[derive(Default, Debug)]
struct State {
    items: Vec<ListItem>,
}

#[derive(Debug, Clone)]
pub enum ListMsg {
    NewItems(Vec<ListItem>),
    Item(usize, ListItemMsg),
}

impl ListView {
    pub fn new(columns_width: Vec<iced::Length>) -> Self {
        Self {
            columns_width,
            state: State::default(),
        }
    }

    fn create_item_view<'a>(&'a self, row: usize, item: &'a ListItem) -> iced::Element<ListMsg> {
        container(
            iced::widget::row(
                (0..self.columns_width.len())
                    .map(|i| {
                        item.get_view(i, self.columns_width[i], 40.0)
                            .map(move |msg| ListMsg::Item(row, msg))
                    })
                    .collect(),
            )
            .spacing(4),
        )
        .style(theme::Container::Custom(
            Box::<style::ContainerBackground>::default(),
        ))
        .into()
    }

    pub fn view<'a>(
        &'a self,
        dialog_row: usize,
        mut content: Option<iced::Element<'a, ListMsg>>,
    ) -> iced::Element<ListMsg> {
        container(scrollable(
            iced::widget::column(
                self.state
                    .items
                    .iter()
                    .enumerate()
                    .map(|(row, item)| {
                        if row == dialog_row && content.is_some() {
                            let header = self.create_item_view(row, item);
                            iced::widget::column![header, content.take().unwrap()].into()
                        } else {
                            self.create_item_view(row, item)
                        }
                    })
                    .collect(),
            )
            .spacing(4)
            .padding([4., 10., 4., 4.]),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Box)
        .into()
    }

    pub fn update(&mut self, message: ListMsg) {
        match message {
            ListMsg::NewItems(items) => self.state.items = items,
            ListMsg::Item(row, msg) => self.state.items[row].update(msg),
        }
    }
}
