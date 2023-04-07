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

pub struct ContainerView {
    list_view: ListView,
    view_state: ViewState,
}

#[derive(Debug)]
enum ContainerMsg {
    View(ListMsg),
}

impl IViewMsg for ContainerMsg {
    fn as_any(&self) -> &dyn Any {
        self
    }
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
            ViewMessage::Init => {
                self.view_state = ViewState::Loading;
                return Command::perform(ContainerView::load(), ViewMessage::Loaded);
            }
            ViewMessage::Selected => {
                if let ViewState::Uninitialized = self.view_state {
                    self.view_state = ViewState::Loading;
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
                    ContainerMsg::View(msg) => {
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
        Box::new(ContainerMsg::View(ListMsg::NewItems(vec![
            Self::container_item("container 1", "Ubuntu"),
            Self::container_item("container 2", "Debian"),
            Self::container_item(
                "mono",
                "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0",
            ),
            Self::container_item("container 22", "Alpine"),
        ])))
    }
}
