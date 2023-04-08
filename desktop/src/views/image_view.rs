use container_core::image::Image;
use iced::Command;
use std::{any::Any, vec};

use crate::{
    controls::{
        list_item::{ListCell, ListItem},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    iview::{IView, IViewMsg, ViewMessage, ViewState},
    provider::Provider,
};

pub struct ImageView {
    list_view: ListView,
    view_state: ViewState,
}

#[derive(Debug)]
enum ImageMsg {
    View(ListMsg),
}

impl IViewMsg for ImageMsg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn list_item(name: String, image: String) -> ListItem {
    ListItem(vec![
        ListCell::IconStatus("image.png"),
        ListCell::TextButton(name),
        ListCell::TextButton(image),
        ListCell::IconToggleButton("play.png", "stop.png"),
        ListCell::IconButton("delete.png"),
    ])
}

fn map_image(imgs: Vec<Image>) -> Box<dyn IViewMsg + Send> {
    let result = imgs
        .into_iter()
        .map(|img| {
            list_item(
                if img.name.is_empty() {
                    img.id
                } else {
                    img.name
                },
                format!("{} MB", img.size as f32 / 1024. / 1024.),
            )
        })
        .collect::<Vec<ListItem>>();
    Box::new(ImageMsg::View(ListMsg::NewItems(result))) as Box<dyn IViewMsg + Send>
}

impl Default for ImageView {
    fn default() -> Self {
        Self {
            list_view: ListView::new(vec![
                iced::Length::Shrink,
                iced::Length::FillPortion(2),
                iced::Length::Fill,
                iced::Length::Shrink,
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
                .view()
                .map(move |msg| ViewMessage::Loaded(Box::new(ImageMsg::View(msg)))),
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => return self.init(),
            ViewMessage::Selected => return self.init(),
            ViewMessage::Error => println!("NOT IMPLEMENED Error"),
            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Loaded(state) => {
                let msg = state
                    .as_any()
                    .downcast_ref::<ImageMsg>()
                    .expect("Wasn't a correct state!");

                match msg {
                    ImageMsg::View(msg) => {
                        match msg {
                            ListMsg::Item(msg) =>
                            /*TODO*/
                            {
                                println!("{}", msg.0);
                            }
                            _ => {
                                self.list_view.update(msg.clone());
                                self.view_state = ViewState::Loaded;
                            }
                        }
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

impl ImageView {
    fn init(&mut self) -> Command<ViewMessage> {
        self.view_state = ViewState::Loading;
        Command::perform(
            Provider::global().image_provider.list(),
            move |imgs| match imgs {
                Some(imgs) => ViewMessage::Loaded(map_image(imgs)),
                None => ViewMessage::Error,
            },
        )
    }
}
