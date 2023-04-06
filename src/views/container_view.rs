use std::{any::Any, thread, time, vec};

use iced::{
    theme,
    widget::{container, scrollable, text},
    Command, Length,
};

use crate::{
    controls::list_item::{ContainerItemMsg, ListCell, ListItem, Status},
    iview::{IView, IViewMsg, ViewMessage, ViewState},
    style,
};

pub struct ContainerView {
    state: State,
    columns_width: Vec<iced::Length>,
}

#[derive(Default, Debug)]
struct State {
    view_state: ViewState,
    containers: Vec<ListItem>,
}

#[derive(Debug)]
enum ContainerMsg {
    State(State),
    Item(ContainerItemMsg),
}

impl IViewMsg for ContainerMsg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn empty_view<'a, Message>() -> iced::widget::Container<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    container(text("Loading..."))
        .style(theme::Container::Box)
        .width(Length::Fill)
        .center_x()
        .center_y()
}

impl Default for ContainerView {
    fn default() -> Self {
        Self {
            columns_width: vec![
                iced::Length::Shrink,
                iced::Length::Fill,
                iced::Length::FillPortion(3),
                iced::Length::Shrink,
                iced::Length::Shrink,
            ],
            state: State::default(),
        }
    }
}

impl IView for ContainerView {
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

    fn view(&self) -> iced::Element<ViewMessage> {
        match self.state.view_state {
            ViewState::Uninitialized => empty_view().into(),
            ViewState::Loading => empty_view().into(),
            ViewState::Loaded => container(scrollable(
                iced::widget::column(
                    self.state
                        .containers
                        .iter()
                        .map(|item| {
                            {
                                container(
                                    iced::widget::row(
                                        (0..5)
                                            .map(|i| {
                                                item.get(
                                                    i,
                                                    self.columns_width[i],
                                                    40.0,
                                                    Status::Running,
                                                )
                                                .map(move |msg| {
                                                    ViewMessage::Loaded(Box::new(
                                                        ContainerMsg::Item(msg),
                                                    ))
                                                })
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
            .padding(2)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::Container::Box)
            .into(),
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => {
                self.state.view_state = ViewState::Loading;
                return Command::perform(ContainerView::load(), ViewMessage::Loaded);
            }
            ViewMessage::Selected => {
                if let ViewState::Uninitialized = self.state.view_state {
                    self.state.view_state = ViewState::Loading;
                    return Command::perform(ContainerView::load(), ViewMessage::Loaded);
                }
            }

            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Loaded(state) => {
                let msg = state
                    .as_any()
                    .downcast_ref::<ContainerMsg>()
                    .expect("Wasn't a correct state!");

                match msg {
                    ContainerMsg::State(s) => {
                        self.state.containers = s.containers.clone();
                        self.state.view_state = ViewState::Loaded;
                    }
                    ContainerMsg::Item(msg) =>
                    /*TODO*/
                    {
                        println!("{}", msg.0)
                    }
                }
            }
        }
        Command::none()
    }

    fn get_badge_number(&self) -> Option<i32> {
        Some(self.state.containers.len() as i32)
    }
}

impl ContainerView {
    fn container_item(name: &'static str, image: &'static str) -> ListItem {
        ListItem(vec![
            ListCell::IconStatus("container.png"),
            ListCell::TextButton(name),
            ListCell::TextButton(image),
            ListCell::IconToggleButton("play.png", "stop.png"),
            ListCell::IconButton("delete.png"),
        ])
    }

    async fn load() -> Box<dyn IViewMsg + Send> {
        dbg!("load called..");
        thread::sleep(time::Duration::from_secs(1));
        Box::new(ContainerMsg::State(State {
            view_state: ViewState::Loaded,
            containers: vec![
                Self::container_item("container 1", "Ubuntu"),
                Self::container_item("container 2", "Debian"),
                Self::container_item(
                    "mono",
                    "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0",
                ),
                Self::container_item("container 22", "Alpine"),
            ],
        }))
    }
}
