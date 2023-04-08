use std::{any::Any, vec};

use container_core::container::Container;
use iced::Command;

use crate::{
    controls::{
        list_item::{ListCell, ListItem, ListItemMsg},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    iview::{IView, IViewMsg, ViewMessage, ViewState},
    provider::Provider,
};

pub struct ContainerView {
    list_view: ListView,
    view_state: ViewState,
}

#[derive(Debug)]
enum ContainerMsg {
    View(ListMsg),
    Started(usize),
    Stopped(usize),
}

static COLUMN_INDEX_STATUS: usize = 0;
static COLUMN_INDEX_ID: usize = 1;
static COLUMN_INDEX_PLAY_STOP: usize = 3;

impl IViewMsg for ContainerMsg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn list_item(name: String, image: String, status: bool) -> ListItem {
    ListItem(vec![
        ListCell::IconStatus("container.png", status),
        ListCell::TextButton(name),
        ListCell::TextButton(image),
        ListCell::IconToggleButton("play.png", "stop.png", status),
        ListCell::IconButton("delete.png"),
    ])
}

fn map_container(list: Vec<Container>) -> Box<dyn IViewMsg + Send> {
    let result = list
        .into_iter()
        .map(|c| {
            list_item(
                if c.name.is_empty() { c.id } else { c.name },
                c.image,
                c.running,
            )
        })
        .collect::<Vec<ListItem>>();
    Box::new(ContainerMsg::View(ListMsg::NewItems(result))) as Box<dyn IViewMsg + Send>
}

impl Default for ContainerView {
    fn default() -> Self {
        Self {
            list_view: ListView::new(vec![
                iced::Length::Shrink,
                iced::Length::Fill,
                iced::Length::FillPortion(3),
                iced::Length::Shrink,
                iced::Length::Shrink,
            ]),
            view_state: ViewState::default(),
        }
    }
}

impl IView for ContainerView {
    fn view(&self) -> iced::Element<ViewMessage> {
        match self.view_state {
            ViewState::Uninitialized => loading_view().into(),
            ViewState::Loading => loading_view().into(),
            ViewState::Loaded => self
                .list_view
                .view()
                .map(move |msg| ViewMessage::Loaded(Box::new(ContainerMsg::View(msg)))),
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => return self.init(),
            ViewMessage::Selected => return self.init(),
            ViewMessage::Error(err) => println!("{:?}", err),
            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Loaded(state) => {
                let msg = state
                    .as_any()
                    .downcast_ref::<ContainerMsg>()
                    .expect("Wasn't a correct state!");

                match msg {
                    ContainerMsg::View(msg) => match msg {
                        ListMsg::Item(row, msg) => {
                            if msg.index == COLUMN_INDEX_PLAY_STOP {
                                let cell = self.list_view.get_cell(*row, COLUMN_INDEX_ID).unwrap();
                                if let ListCell::TextButton(name) = cell {
                                    let name = name.to_string();
                                    let row = *row;
                                    return if msg.state {
                                        Command::perform(
                                            Provider::global().stop_container(name),
                                            move |e| match e {
                                                Ok(_) => ViewMessage::Loaded(Box::new(
                                                    ContainerMsg::Stopped(row),
                                                )),
                                                Err(err) => ViewMessage::Error(err),
                                            },
                                        )
                                    } else {
                                        Command::perform(
                                            Provider::global().start_container(name),
                                            move |e| match e {
                                                Ok(_) => ViewMessage::Loaded(Box::new(
                                                    ContainerMsg::Started(row),
                                                )),
                                                Err(err) => ViewMessage::Error(err),
                                            },
                                        )
                                    };
                                }
                            } else {
                                println!("clicked {}", msg.index);
                            }
                        }
                        _ => {
                            self.list_view.update(msg.clone());
                            self.view_state = ViewState::Loaded;
                        }
                    },
                    ContainerMsg::Started(row) => {
                        self.list_view.update(ListMsg::Item(
                            *row,
                            ListItemMsg::new(COLUMN_INDEX_PLAY_STOP, true),
                        ));
                        self.list_view.update(ListMsg::Item(
                            *row,
                            ListItemMsg::new(COLUMN_INDEX_STATUS, true),
                        ));
                    }
                    ContainerMsg::Stopped(row) => {
                        self.list_view.update(ListMsg::Item(
                            *row,
                            ListItemMsg::new(COLUMN_INDEX_PLAY_STOP, false),
                        ));
                        self.list_view.update(ListMsg::Item(
                            *row,
                            ListItemMsg::new(COLUMN_INDEX_STATUS, false),
                        ));
                    }
                }
            }
        }
        Command::none()
    }

    fn get_badge_number(&self) -> Option<i32> {
        Some(self.list_view.get_item_count() as i32)
    }
}

impl ContainerView {
    fn init(&mut self) -> Command<ViewMessage> {
        self.view_state = ViewState::Loading;
        Command::perform(
            Provider::global().list_containers(),
            move |imgs| match imgs {
                Ok(list) => ViewMessage::Loaded(map_container(list)),
                Err(err) => ViewMessage::Error(err),
            },
        )
    }
}
