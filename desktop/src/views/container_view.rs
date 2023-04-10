use std::{any::Any, vec};

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
static COLUMN_INDEX_PLAY_STOP: usize = 4;

fn list_item(c: &Container) -> ListItem {
    let name = if c.name.is_empty() {
        c.id.clone()
    } else {
        c.name.clone()
    };
    ListItem(vec![
        ListCell::IconStatus("container.png", c.running),
        ListCell::TextButton(name),
        ListCell::TextButton(c.name.clone()),
        ListCell::TextButton(c.status.clone()),
        ListCell::IconButton(if c.running { "stop.png" } else { "play.png" }),
        ListCell::IconButton("delete.png"),
    ])
}

fn map_container(list: &[Container]) -> Vec<ListItem> {
    list.iter().map(list_item).collect()
}

impl Default for ContainerView {
    fn default() -> Self {
        Self {
            list_view: ListView::new(vec![
                iced::Length::Shrink,
                iced::Length::FillPortion(4),
                iced::Length::FillPortion(10),
                iced::Length::FillPortion(5),
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
            ViewMessage::Init => {
                self.view_state = ViewState::Loading;
                return self.init();
            }
            ViewMessage::Selected => return self.init(),
            ViewMessage::Error(err) => println!("{:?}", err),
            ViewMessage::Update => return self.init(),
            ViewMessage::Loaded(state) => return self.process_loaded_msg(state),
            ViewMessage::Unselected => (),
            ViewMessage::UpdateBadge(_) => (),
        }
        Command::none()
    }
}

impl ContainerView {
    fn init(&mut self) -> Command<ViewMessage> {
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

    fn process_loaded_msg(&mut self, state: Box<dyn Any>) -> Command<ViewMessage> {
        let state = state
            .downcast::<ContainerMsg>()
            .expect("expected box to be ContainerMsg");
        match *state {
            ContainerMsg::View(msg) => match msg {
                ListMsg::Item(row, ListItemMsg::Clicked(col)) => {
                    if col == COLUMN_INDEX_PLAY_STOP {
                        self.set_busy_cells(row);
                        let container = &self.containers[row];
                        let id = container.id.clone();
                        return if container.running {
                            Command::perform(
                                Provider::global().stop_container(id),
                                move |e| match e {
                                    Ok(_) => {
                                        ViewMessage::Loaded(Box::new(ContainerMsg::Stopped(row)))
                                    }
                                    Err(err) => ViewMessage::Error(err),
                                },
                            )
                        } else {
                            Command::perform(Provider::global().start_container(id), move |e| {
                                match e {
                                    Ok(_) => {
                                        ViewMessage::Loaded(Box::new(ContainerMsg::Started(row)))
                                    }
                                    Err(err) => ViewMessage::Error(err),
                                }
                            })
                        };
                    } else {
                        println!("clicked {row}, {col}");
                    }
                }
                _ => self.list_view.update(msg),
            },
            ContainerMsg::Started(row) => self.update_running_state(row, true),
            ContainerMsg::Stopped(row) => self.update_running_state(row, false),
            ContainerMsg::NewContainers(list) => return self.diff_apply(list),
        }
        Command::none()
    }

    fn update_running_state(&mut self, row: usize, running: bool) {
        self.containers[row].running = running;
        self.replace_cell(
            row,
            COLUMN_INDEX_PLAY_STOP,
            ListCell::IconButton(if running { "stop.png" } else { "play.png" }),
        );
        self.replace_cell(
            row,
            COLUMN_INDEX_STATUS,
            ListCell::IconStatus("container.png", running),
        );
    }

    fn diff_apply(&mut self, list: Vec<Container>) -> Command<ViewMessage> {
        self.view_state = ViewState::Loaded;
        let mut new_changes = false;
        if list.len() == self.containers.len() {
            for (row, new) in list.iter().enumerate() {
                if new.id != self.containers[row].id {
                    new_changes = true;
                    break;
                }

                if new.running != self.containers[row].running {
                    self.update_running_state(row, new.running);
                }
            }
            if !new_changes {
                return Command::none();
            }
        }

        let items = map_container(&list);
        self.containers = list;
        self.list_view.update(ListMsg::NewItems(items));
        let badge = self.containers.len() as i32;
        Command::perform(async move { badge }, ViewMessage::UpdateBadge)
    }
}
