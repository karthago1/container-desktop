use std::vec;

use container_core::container::Container;
use iced::Command;

use crate::{
    controls::{
        list_item::{ListCell, ListItem, ListItemMsg},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    iview::{IView, ViewMessage, ViewState},
    provider::Provider,
};

pub struct ContainerView {
    list_view: ListView,
    view_state: ViewState,
    containers: Vec<Container>,
}

#[derive(Debug)]
enum ContainerMsg {
    View(ListMsg),
    Started(usize),
    Stopped(usize),
    NewContainers(Vec<Container>),
}

static COLUMN_INDEX_STATUS: usize = 0;
static COLUMN_INDEX_PLAY_STOP: usize = 3;

fn list_item(name: String, image: String, status: bool) -> ListItem {
    ListItem(vec![
        ListCell::IconStatus("container.png", status),
        ListCell::TextButton(name),
        ListCell::TextButton(image),
        ListCell::IconButton(if status { "stop.png" } else { "play.png" }),
        ListCell::IconButton("delete.png"),
    ])
}

fn map_container(list: &[Container]) -> Vec<ListItem> {
    list.iter()
        .map(|c| {
            list_item(
                if c.name.is_empty() {
                    c.id.clone()
                } else {
                    c.name.clone()
                },
                c.image.clone(),
                c.running,
            )
        })
        .collect()
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
            containers: vec![],
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
                let state = state
                    .downcast::<ContainerMsg>()
                    .expect("expected box to be ContainerMsg");
                match *state {
                    ContainerMsg::View(msg) => match msg {
                        ListMsg::Item(row, ListItemMsg::Clicked(col)) => {
                            if col == COLUMN_INDEX_PLAY_STOP {
                                self.set_busy_cells(row);
                                let container = &self.containers[row];
                                let name = container.name.clone();
                                return if container.running {
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
                            } else {
                                println!("clicked {row}, {col}");
                            }
                        }
                        _ => self.list_view.update(msg),
                    },
                    ContainerMsg::Started(row) => {
                        self.containers[row].running = true;
                        self.replace_cell(
                            row,
                            COLUMN_INDEX_PLAY_STOP,
                            ListCell::IconButton("stop.png"),
                        );
                        self.replace_cell(
                            row,
                            COLUMN_INDEX_STATUS,
                            ListCell::IconStatus("container.png", true),
                        );
                    }
                    ContainerMsg::Stopped(row) => {
                        self.containers[row].running = false;
                        self.replace_cell(
                            row,
                            COLUMN_INDEX_PLAY_STOP,
                            ListCell::IconButton("play.png"),
                        );
                        self.replace_cell(
                            row,
                            COLUMN_INDEX_STATUS,
                            ListCell::IconStatus("container.png", false),
                        );
                    }
                    ContainerMsg::NewContainers(list) => {
                        let items = map_container(&list);
                        self.containers = list;
                        self.list_view.update(ListMsg::NewItems(items));
                        self.view_state = ViewState::Loaded;
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
                Ok(list) => ViewMessage::Loaded(Box::new(ContainerMsg::NewContainers(list))),
                Err(err) => ViewMessage::Error(err),
            },
        )
    }

    fn replace_cell(&mut self, row: usize, col: usize, new_cell: ListCell) {
        self.list_view
            .update(ListMsg::Item(row, ListItemMsg::ChangeCell(col, new_cell)));
    }

    fn set_busy_cells(&mut self, row: usize) {
        let new_cell = ListCell::IconStatus("hourglass.png", true);
        self.replace_cell(row, COLUMN_INDEX_PLAY_STOP, new_cell.clone());
        self.replace_cell(row, COLUMN_INDEX_STATUS, new_cell);
    }
}
