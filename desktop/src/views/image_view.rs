use container_core::image::Image;
use iced::Command;
use std::vec;

use crate::{
    controls::{
        icons,
        list_item::{ListCell, ListItem},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    provider::Provider,
};

use super::{view_result, IView, ViewError, ViewMessage, ViewResult, ViewState};

pub struct ImageView {
    list_view: ListView,
    view_state: ViewState,
}

#[derive(Debug)]
enum ImageMsg {
    View(ListMsg),
    NewImages(ViewResult<Vec<Image>>),
}

fn list_item(name: String, image: String, status: bool) -> ListItem {
    ListItem(vec![
        ListCell::IconStatus(icons::ICON_IMAGE, status),
        ListCell::TextButton(name, 0),
        ListCell::TextButton(image, 0),
        ListCell::IconButton(icons::ICON_DELETE, 0),
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

impl Default for ImageView {
    fn default() -> Self {
        Self {
            list_view: ListView::new(vec![
                iced::Length::Shrink,
                iced::Length::FillPortion(2),
                iced::Length::Fill,
                iced::Length::Shrink,
            ]),
            view_state: ViewState::default(),
        }
    }
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
                    ImageMsg::View(msg) => {
                        match msg {
                            ListMsg::Item(row, _msg) =>
                            /*TODO*/
                            {
                                println!("event {row}");
                            }
                            _ => {
                                self.list_view.update(msg);
                            }
                        }
                    }
                    ImageMsg::NewImages(list) => match list {
                        Ok(list) => {
                            let msg = map_image(&list);
                            self.list_view.update(ListMsg::NewItems(msg));
                            self.view_state = ViewState::Loaded;
                            let badge = list.len() as i32;
                            return Command::perform(
                                async move { badge },
                                ViewMessage::UpdateBadge,
                            );
                        }
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
    fn create_load_cmd(&self) -> Command<ViewMessage> {
        Command::perform(Provider::global().list_images(), |imgs| {
            ViewMessage::Loaded(Box::new(ImageMsg::NewImages(view_result!(imgs))))
        })
    }

    fn init(&mut self) -> Command<ViewMessage> {
        self.view_state = ViewState::Loading;
        self.create_load_cmd()
    }
}
