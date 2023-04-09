use std::any::Any;

use container_core::Error;
use iced::Command;

#[derive(Debug)]
pub enum ViewMessage {
    Init,
    Loaded(Box<dyn Any + Send>),
    Selected,
    Unselected,
    Error(Error),
}

#[derive(Default, Debug)]
pub enum ViewState {
    #[default]
    Uninitialized,
    Loading,
    Loaded,
}

pub trait IView {
    fn view(&self) -> iced::Element<ViewMessage>;
    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage>;
    fn get_badge_number(&self) -> Option<i32>;
}
