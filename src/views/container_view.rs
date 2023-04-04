use std::{any::Any, thread, time, vec};

use iced::{
    theme,
    widget::{column, container, scrollable, text},
    Command, Length,
};

use crate::{
    container_item::ContainerItem,
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
            ViewState::Loaded => container(scrollable(column(
                self.state
                    .containers
                    .iter()
                    .map(|item| item.view().into())
                    .collect(),
            ))),
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
}

impl ContainerView {
    async fn load() -> Box<dyn IViewState + Send> {
        dbg!("load called..");
        thread::sleep(time::Duration::from_secs(3));
        Box::new(State {
            view_state: ViewState::Loaded,
            containers: vec![
                ContainerItem::new("12345".to_string()),
                ContainerItem::new("new Contrainer".to_string()),
            ],
        })
    }
}
