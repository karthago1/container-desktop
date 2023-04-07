use core::CorePlugin;
use std::{
    any::Any,
    sync::{Arc, Mutex},
    vec,
};

use iced::Command;

use crate::{
    controls::{
        list_item::{ListCell, ListItem},
        list_view::{ListMsg, ListView},
        loading_view,
    },
    iview::{IView, IViewMsg, ViewMessage, ViewState},
};

pub struct ImageView {
    provider: Arc<Mutex<Box<CorePlugin>>>,
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
            ViewMessage::Init => {
                self.view_state = ViewState::Loading;
                return Command::perform(
                    ImageView::load(self.provider.clone()),
                    ViewMessage::Loaded,
                );
            }
            ViewMessage::Selected => {
                if let ViewState::Uninitialized = self.view_state {
                    self.view_state = ViewState::Loading;
                    return Command::perform(
                        ImageView::load(self.provider.clone()),
                        ViewMessage::Loaded,
                    );
                }
            }

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
    pub fn new(provider: Arc<Mutex<Box<CorePlugin>>>) -> Self {
        Self {
            provider,
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

    fn list_item(name: String, image: String) -> ListItem {
        ListItem(vec![
            ListCell::IconStatus("image.png"),
            ListCell::TextButton(name),
            ListCell::TextButton(image),
            ListCell::IconToggleButton("play.png", "stop.png"),
            ListCell::IconButton("delete.png"),
        ])
    }

    async fn load(provider: Arc<Mutex<Box<CorePlugin>>>) -> Box<dyn IViewMsg + Send> {
        let list = {
            let lock = provider.lock().unwrap();
            lock.image_provider.list()
        };

        let result: Vec<ListItem> = list
            .into_iter()
            .map(|img| {
                Self::list_item(
                    if img.name.is_empty() {
                        img.id
                    } else {
                        img.name
                    },
                    format!("{} MB", img.size as f32 / 1024. / 1024.),
                )
            })
            .collect();

        Box::new(ImageMsg::View(ListMsg::NewItems(result)))
    }
}
