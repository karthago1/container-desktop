use std::any::Any;

use iced::{widget::Container, Command, Renderer};

#[derive(Debug)]
pub enum ViewMessage {
    Init,
    Loaded(Box<dyn IViewState + Send>),
    Selected,
    Unselected,
}

#[derive(Default, Debug)]
pub enum ViewState {
    #[default]
    Uninitialized,
    Loading,
    Loaded,
}

pub trait IViewState: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait IView {
    fn view<'a>(&self) -> Container<'a, crate::Message, Renderer>;
    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage>;
    fn get_badge_number(&self) -> Option<i32>;
}
