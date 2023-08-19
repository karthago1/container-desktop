use std::{any::Any, vec};

use container_core::{container::Container, CorePlugin};
use iced::Command;

use crate::{
    controls::{
        icons,
        list_item::{ListCell, ListItem, ListItemMsg},
        list_view::{ListMsg, ListView},
        loading_view,
        ui::icon_button,
    },
    provider::Provider,
    views::{view_msg, view_result, IView, ViewError, ViewMessage, ViewState},
};

use super::ViewResult;

pub struct ContainerView {
    plugin_index: usize,
    list_view: ListView,
    view_state: ViewState,
    containers: Vec<Container>,
    detail_view: DetailView,
    clone_name: String,
    error: Option<ViewError>,
}

enum DetailView {
    None,
    Clone(usize),
    //Info(usize),
}

#[derive(Debug, Clone)]
enum ContainerMsg {
    View(ListMsg),
    Started(usize, ViewResult<()>),
    Stopped(usize, ViewResult<()>),
    NewContainers(ViewResult<Vec<Container>>),
    Cloned(ViewResult<()>),
    Deleted(usize, ViewResult<()>),
    CloseDialog,
}

static COLUMN_INDEX_STATUS: usize = 0;
static COLUMN_INDEX_PLAY_STOP: usize = 4;
static COLUMN_INDEX_DELETE: usize = 5;

static ACTION_EMPTY: u32 = 0;
static ACTION_STOP_START: u32 = 1;
static ACTION_CLONE: u32 = 3;
static ACTION_SHOW_CLONE_DIALOG: u32 = 4;
static ACTION_DELETE: u32 = 5;

impl IView for ContainerView {
    fn view(&self) -> iced::Element<ViewMessage> {
        match self.view_state {
            ViewState::Uninitialized => loading_view().into(),
            ViewState::Loading => loading_view().into(),
            ViewState::Loaded => {
                let (row, view) = self.create_detail_view();

                let content = self.list_view.view(row, view).map(ContainerMsg::View);

                let result = match &self.error {
                    Some(err) => {
                        crate::controls::error_dialog(&err.0, content, ContainerMsg::CloseDialog)
                    }
                    None => content,
                };

                result.map(|msg| view_msg!(msg))
            }
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => {
                self.view_state = ViewState::Loading;
                return self.init();
            }
            ViewMessage::Selected => return self.init(),
            //ViewMessage::Error(err) => println!("{:?}", err),
            ViewMessage::Update => return self.init(),
            ViewMessage::Loaded(state) => return self.process_loaded_msg(state),
            ViewMessage::Unselected => (),
            ViewMessage::UpdateBadge(_) => (),
        }
        Command::none()
    }
}

impl ContainerView {
    pub fn new(plugin_index: usize) -> Self {
        let mut item_lengths: Vec<iced::Length> = vec![
            iced::Length::Shrink,
            iced::Length::FillPortion(5),
            iced::Length::FillPortion(10),
            iced::Length::FillPortion(5),
            iced::Length::Shrink,
            iced::Length::Shrink,
        ];

        if Provider::global(plugin_index).is_clone_supported() {
            item_lengths.push(iced::Length::Shrink);
        }

        Self {
            plugin_index,
            list_view: ListView::new(item_lengths),
            view_state: ViewState::default(),
            containers: vec![],
            detail_view: DetailView::None,
            clone_name: "".to_string(),
            error: None,
        }
    }

    fn plugin(&self) -> &'static dyn CorePlugin {
        Provider::global(self.plugin_index)
    }

    fn init(&mut self) -> Command<ViewMessage> {
        Command::perform(self.plugin().list_containers(), |imgs| {
            let res = view_result!(imgs);
            view_msg!(ContainerMsg::NewContainers(res))
        })
    }

    fn create_detail_view(&self) -> (usize, Option<iced::Element<ListMsg>>) {
        match self.detail_view {
            DetailView::None => (0, None),
            //DetailView::Info(_row) => (0, None),
            DetailView::Clone(row) => (
                row,
                Some(
                    iced::widget::container(
                        iced::widget::row![
                            iced::widget::text_input("Clone Name", &self.clone_name).on_input(
                                move |value| {
                                    ListMsg::Item(row, ListItemMsg::TextChanged(0, value))
                                }
                            ),
                            icon_button(icons::ICON_DONE).on_press(ListMsg::Item(
                                row,
                                ListItemMsg::Clicked(0, ACTION_CLONE)
                            ))
                        ]
                        .width(300)
                        .height(40),
                    )
                    .style(iced::theme::Container::Box)
                    .center_x()
                    .width(iced::Length::Fill)
                    .into(),
                ),
            ),
        }
    }

    fn replace_cell(&mut self, row: usize, col: usize, new_cell: ListCell) {
        self.list_view
            .update(ListMsg::Item(row, ListItemMsg::ChangeCell(col, new_cell)));
    }

    fn set_busy_cell(&mut self, row: usize, col: usize) {
        let new_cell = ListCell::IconStatus(icons::ICON_HOURGLASS, true);
        self.replace_cell(row, col, new_cell.clone());
        self.replace_cell(row, COLUMN_INDEX_STATUS, new_cell);
    }

    fn start_stop_container_cmd(&mut self, row: usize, col: usize) -> Command<ViewMessage> {
        self.set_busy_cell(row, col);
        let container = &self.containers[row];
        let id = container.id.clone();
        if container.running {
            Command::perform(self.plugin().stop_container(id), move |e| {
                view_msg!(ContainerMsg::Stopped(row, view_result!(e)))
            })
        } else {
            Command::perform(self.plugin().start_container(id), move |e| {
                view_msg!(ContainerMsg::Started(row, view_result!(e)))
            })
        }
    }

    fn show_detail_view_cmd(&mut self, row: usize) -> Command<ViewMessage> {
        self.detail_view = match self.detail_view {
            DetailView::Clone(old_row) => {
                if old_row == row {
                    DetailView::None
                } else {
                    DetailView::Clone(row)
                }
            }
            /*DetailView::Info(_) => {
                self.clone_name.clear();
                DetailView::Clone(row);
            }*/
            DetailView::None => {
                self.clone_name.clear();
                DetailView::Clone(row)
            }
        };

        Command::none()
    }

    fn clone_cmd(&self, row: usize) -> Command<ViewMessage> {
        let container = &self.containers[row];
        Command::perform(
            self.plugin()
                .clone_container(container.id.clone(), self.clone_name.clone()),
            |e| view_msg!(ContainerMsg::Cloned(view_result!(e))),
        )
    }

    fn delete_cmd(&mut self, row: usize, col: usize) -> Command<ViewMessage> {
        self.set_busy_cell(row, col);
        let container = &self.containers[row];
        Command::perform(
            self.plugin().remove_container(container.id.clone()),
            move |e| view_msg!(ContainerMsg::Deleted(row, view_result!(e))),
        )
    }

    fn process_loaded_msg(&mut self, state: Box<dyn Any>) -> Command<ViewMessage> {
        let state = state
            .downcast::<ContainerMsg>()
            .expect("expected box to be ContainerMsg");
        match *state {
            ContainerMsg::View(msg) => match msg {
                ListMsg::Item(row, ListItemMsg::Clicked(col, action)) => {
                    return if action == ACTION_STOP_START {
                        self.start_stop_container_cmd(row, col)
                    } else if action == ACTION_SHOW_CLONE_DIALOG {
                        self.show_detail_view_cmd(row)
                    } else if action == ACTION_CLONE {
                        self.clone_cmd(row)
                    } else if action == ACTION_DELETE {
                        self.delete_cmd(row, col)
                    } else {
                        println!("clicked {row}, {col}");
                        Command::none()
                    }
                }
                ListMsg::Item(_row, ListItemMsg::TextChanged(_col, value)) => {
                    self.clone_name = value
                }
                _ => self.list_view.update(msg),
            },
            ContainerMsg::NewContainers(res) => match res {
                Ok(list) => return self.diff_apply(list),
                Err(err) => self.error = Some(err),
            },
            ContainerMsg::Started(row, res) => {
                self.update_running_state(row, res.is_ok());
                if let Err(err) = res {
                    self.error = Some(err);
                }
            }
            ContainerMsg::Stopped(row, res) => {
                self.update_running_state(row, res.is_err());
                if let Err(err) = res {
                    self.error = Some(err);
                }
            }
            ContainerMsg::Cloned(res) => {
                self.detail_view = DetailView::None;
                if let Err(err) = res {
                    self.error = Some(err);
                }
            }
            ContainerMsg::Deleted(row, res) => {
                if let Err(err) = res {
                    self.update_delete_button(row);
                    self.error = Some(err);
                }
            }
            ContainerMsg::CloseDialog => _ = self.error.take(),
        }
        Command::none()
    }

    fn update_delete_button(&mut self, row: usize) {
        self.replace_cell(
            row,
            COLUMN_INDEX_DELETE,
            ListCell::IconButton(icons::ICON_DELETE, ACTION_DELETE),
        );
        self.replace_cell(
            row,
            COLUMN_INDEX_STATUS,
            ListCell::IconStatus(icons::ICON_CONTAINER, self.containers[row].running),
        );
    }

    fn update_running_state(&mut self, row: usize, running: bool) {
        self.containers[row].running = running;
        self.replace_cell(
            row,
            COLUMN_INDEX_PLAY_STOP,
            ListCell::IconButton(
                if running {
                    icons::ICON_STOP
                } else {
                    icons::ICON_PLAY
                },
                ACTION_STOP_START,
            ),
        );
        self.replace_cell(
            row,
            COLUMN_INDEX_STATUS,
            ListCell::IconStatus(icons::ICON_CONTAINER, running),
        );
    }

    fn list_item(&self, c: &Container) -> ListItem {
        let name = if c.name.is_empty() {
            c.id.clone()
        } else {
            c.name.clone()
        };

        let mut cells: Vec<ListCell> = vec![
            ListCell::IconStatus(icons::ICON_CONTAINER, c.running),
            ListCell::TextButton(name, ACTION_EMPTY),
            ListCell::TextButton(c.image.clone(), ACTION_EMPTY),
            ListCell::TextButton(c.status.clone(), ACTION_EMPTY),
            ListCell::IconButton(
                if c.running {
                    icons::ICON_STOP
                } else {
                    icons::ICON_PLAY
                },
                ACTION_STOP_START,
            ),
            ListCell::IconButton(icons::ICON_DELETE, ACTION_DELETE),
        ];

        if self.plugin().is_clone_supported() {
            cells.push(ListCell::IconButton(
                icons::ICON_CLONE,
                ACTION_SHOW_CLONE_DIALOG,
            ));
        }

        ListItem(cells)
    }

    fn map_container(&self, list: &[Container]) -> Vec<ListItem> {
        list.iter().map(|e| self.list_item(e)).collect()
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

        let items = self.map_container(&list);
        self.containers = list;
        self.list_view.update(ListMsg::NewItems(items));
        let badge = self.containers.len() as i32;
        Command::perform(async move { badge }, ViewMessage::UpdateBadge)
    }
}
