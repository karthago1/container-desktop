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

    /*fn view2(&self) -> iced::Element<ViewMessage> {
        match self.state.view_state {
            ViewState::Uninitialized => empty_view().into(),
            ViewState::Loading => empty_view().into(),
            ViewState::Loaded => container(scrollable(iced::widget::row(
                (0usize..5)
                    .map(|i| {
                        {
                            iced::widget::column(
                                self.state
                                    .containers
                                    .iter()
                                    .map(|item| {
                                        item.get(i, 32.0, Status::Running).map(move |msg| {
                                            ViewMessage::Loaded(Box::new(ContainerMsg::Item(msg)))
                                        })
                                    })
                                    .collect(),
                            )
                            .spacing(4)
                        }
                        .into()
                    })
                    .collect(),
            )))
            .padding(2)
            .width(Length::Fill)
            .style(theme::Container::Box)
            .into(),
        }
    }*/

    pub fn view(&self) -> iced::Element<ListMsg> {
        container(scrollable(
            iced::widget::column(
                self.state
                    .items
                    .iter()
                    .enumerate()
                    .map(|(row, item)| {
                        {
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
                        }
                        .into()
                    })
                    .collect(),
            )
            .spacing(4),
        ))
        .padding(4)
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
