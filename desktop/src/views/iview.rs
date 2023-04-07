use std::any::Any;

use iced::Command;

#[derive(Debug)]
pub enum ViewMessage {
    Init,
    Loaded(Box<dyn IViewMsg + Send>),
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

pub trait IViewMsg: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait IView {
    fn view(&self) -> iced::Element<ViewMessage>;
    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage>;
    fn get_badge_number(&self) -> Option<i32>;
}