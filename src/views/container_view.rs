use std::{any::Any, thread, time, vec};

use iced::{
    theme,
    widget::{container, scrollable, text},
    Command, Length,
};

use crate::{
    container_item::{ContainerItem, Status},
    iview::{IView, IViewState, ViewMessage, ViewState},
};

#[derive(Default)]
pub struct ContainerView {
    state: State,
}

#[derive(Default, Debug)]
struct State {
    view_state: ViewState,
    containers: Vec<ContainerItem>,
}
impl IViewState for State {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn empty_view<'a, Message>() -> iced::widget::Container<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    container(text("Loading..."))
        .style(theme::Container::Box)
        .width(Length::Fill)
        .center_x()
        .center_y()
}

impl IView for ContainerView {
    fn view<'a>(&self) -> iced::widget::Container<'a, crate::Message, iced::Renderer> {
        match self.state.view_state {
            ViewState::Uninitialized => empty_view(),
            ViewState::Loading => empty_view(),
            ViewState::Loaded => container(scrollable(iced::widget::row(
                (0usize..3)
                    .map(|i| {
                        {
                            iced::widget::column(
                                self.state
                                    .containers
                                    .iter()
                                    .map(|item| item.get(i, 32.0).into())
                                    .collect(),
                            )
                            .spacing(4)
                        }
                        .into()
                    })
                    .collect(),
            )))
            .padding(2)
            .width(Length::Fill)
            .style(theme::Container::Box),
        }
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => {
                self.state.view_state = ViewState::Loading;
                return Command::perform(ContainerView::load(), ViewMessage::Loaded);
            }
            ViewMessage::Selected => match self.state.view_state {
                ViewState::Uninitialized => {
                    self.state.view_state = ViewState::Loading;
                    return Command::perform(ContainerView::load(), ViewMessage::Loaded);
                }
                _ => (),
            },
            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Loaded(state) => {
                let my_state = state
                    .as_any()
                    .downcast_ref::<State>()
                    .expect("Wasn't a correct state!");

                self.state.containers = my_state.containers.clone();
                self.state.view_state = ViewState::Loaded;
            }
        }
        Command::none()
    }

    fn get_badge_number(&self) -> Option<i32> {
        Some(self.state.containers.len() as i32)
    }
}

impl ContainerView {
    async fn load() -> Box<dyn IViewState + Send> {
        dbg!("load called..");
        thread::sleep(time::Duration::from_secs(1));
        Box::new(State {
            view_state: ViewState::Loaded,
            containers: vec![
                ContainerItem::new("12345".to_string(), Status::Running, "iotcore".to_string()),
                ContainerItem::new(
                    "new Contrainer".to_string(),
                    Status::Exited,
                    "sha256:ea49d6ddc21b6ca2e00b002e7f254325df0ff7eb1a9eb8a9a15ad151eda39be0"
                        .to_string(),
                ),
                ContainerItem::new("mono".to_string(), Status::Exited, "Ubuntu".to_string()),
            ],
        })
    }
}
