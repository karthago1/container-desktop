use std::{any::Any, thread, time, vec};

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
            ViewMessage::Init => {
                self.view_state = ViewState::Loading;
                return Command::perform(ImageView::load(), ViewMessage::Loaded);
            }
            ViewMessage::Selected => {
                if let ViewState::Uninitialized = self.view_state {
                    self.view_state = ViewState::Loading;
                    return Command::perform(ImageView::load(), ViewMessage::Loaded);
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
    fn list_item(name: &'static str, image: &'static str) -> ListItem {
        ListItem(vec![
            ListCell::IconStatus("image.png"),
            ListCell::TextButton(name),
            ListCell::TextButton(image),
            ListCell::IconToggleButton("play.png", "stop.png"),
            ListCell::IconButton("delete.png"),
        ])
    }

    async fn load() -> Box<dyn IViewMsg + Send> {
        dbg!("load called..");
        thread::sleep(time::Duration::from_secs(1));
        Box::new(ImageMsg::View(ListMsg::NewItems(vec![
            Self::list_item("Ubuntu", "907.13 MB"),
            Self::list_item("Debian", "713.15 MB"),
            Self::list_item(
                "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0",
                "713.15 MB",
            ),
            Self::list_item("Alpine", "13.15 MB"),
        ])))
    }
}
