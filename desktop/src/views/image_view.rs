use container_core::{image::Image, CorePlugin};
use iced::Command;
use std::vec;

use crate::{
    controls::{
        icons,
        list_item::{ListCell, ListItem, ListItemMsg},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    provider::Provider,
};

use super::{view_msg, view_result, IView, ViewError, ViewMessage, ViewResult, ViewState};

pub struct ImageView {
    plugin_index: usize,
    list_view: ListView,
    view_state: ViewState,
    images: Vec<Image>,
}

#[derive(Debug)]
enum ImageMsg {
    View(ListMsg),
    NewImages(ViewResult<Vec<Image>>),
    Exported(ViewResult<()>),
}

static ACTION_EMPTY: u32 = 0;
static ACTION_EXPORT: u32 = 1;
static ACTION_DELETE: u32 = 2;

fn list_item(name: String, image: String, status: bool) -> ListItem {
    ListItem(vec![
        ListCell::IconStatus(icons::ICON_IMAGE, status),
        ListCell::TextButton(name, ACTION_EMPTY),
        ListCell::TextButton(image, ACTION_EMPTY),
        ListCell::IconButton(icons::ICON_DELETE, ACTION_DELETE),
        ListCell::IconButton(icons::ICON_SAVE_AS, ACTION_EXPORT),
    ])
}

fn map_image(imgs: &[Image]) -> Vec<ListItem> {
    imgs.iter()
        .map(|img| {
            list_item(
                if img.name.is_empty() {
                    img.id.clone()
                } else {
                    img.name.clone()
                },
                format!("{} MB", img.size as f32 / 1024. / 1024.),
                false,
            )
        })
        .collect()
}

impl IView for ImageView {
    fn view(&self) -> iced::Element<ViewMessage> {
        match self.view_state {
            ViewState::Uninitialized => loading_view().into(),
            ViewState::Loading => loading_view().into(),
            ViewState::Loaded => self
                .list_view
                .view(0, None)
                .map(move |msg| ViewMessage::Loaded(Box::new(ImageMsg::View(msg)))),
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => return self.init(),
            ViewMessage::Selected => return self.init(),
            ViewMessage::Update => println!("NOT IMPLEMENED Update"), //return self.create_load_cmd(),
            ViewMessage::Loaded(state) => {
                let state = state
                    .downcast::<ImageMsg>()
                    .expect("expected box to be ImageMsg");

                match *state {
                    ImageMsg::View(msg) => match msg {
                        ListMsg::Item(row, ListItemMsg::Clicked(col, action)) => {
                            return if action == ACTION_EXPORT {
                                self.export_image(row, col)
                            } else {
                                println!("clicked {row}, {col}");
                                Command::none()
                            }
                        }
                        _ => {
                            self.list_view.update(msg);
                        }
                    },
                    ImageMsg::NewImages(list) => match list {
                        Ok(list) => {
                            let msg = map_image(&list);
                            self.list_view.update(ListMsg::NewItems(msg));
                            self.view_state = ViewState::Loaded;
                            let badge = list.len() as i32;
                            self.images = list;
                            return Command::perform(
                                async move { badge },
                                ViewMessage::UpdateBadge,
                            );
                        }
                        Err(err) => println!("{:?}", err),
                    },
                    ImageMsg::Exported(res) => match res {
                        Ok(_) => (),
                        Err(err) => println!("{:?}", err),
                    },
                }
            }
            ViewMessage::Unselected => (),
            ViewMessage::UpdateBadge(_) => (),
        }
        Command::none()
    }
}

impl ImageView {
    pub fn new(plugin_index: usize) -> Self {
        Self {
            plugin_index,
            list_view: ListView::new(vec![
                iced::Length::Shrink,
                iced::Length::FillPortion(2),
                iced::Length::Fill,
                iced::Length::Shrink,
                iced::Length::Shrink,
            ]),
            view_state: ViewState::default(),
            images: vec![],
        }
    }

    fn export_image(&mut self, row: usize, col: usize) -> Command<ViewMessage> {
        //self.set_busy_cell(row, col);
        let image = &self.images[row];
        let id = image.id.clone();
        Command::perform(
            self.plugin()
                .export_image(id, "/tmp/exported-image.tar".to_string()),
            move |e| view_msg!(ImageMsg::Exported(view_result!(e))),
        )
    }

    fn create_load_cmd(&self) -> Command<ViewMessage> {
        Command::perform(self.plugin().list_images(), |imgs| {
            ViewMessage::Loaded(Box::new(ImageMsg::NewImages(view_result!(imgs))))
        })
    }

    fn plugin(&self) -> &'static dyn CorePlugin {
        Provider::global(self.plugin_index)
    }

    fn init(&mut self) -> Command<ViewMessage> {
        self.view_state = ViewState::Loading;
        self.create_load_cmd()
    }
}
