use iced::Command;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct ViewError(pub String);

pub type ViewResult<T, E = ViewError> = core::result::Result<T, E>;

macro_rules! view_result {
    ($e:expr) => {
        match $e {
            Ok(x) => Ok(x),
            Err(err) => Err(ViewError(err.to_string())),
        }
    };
}

macro_rules! view_msg {
    ($msg:expr) => {
        ViewMessage::Loaded(Box::new($msg))
    };
}

pub(crate) use view_result;
pub(crate) use view_msg;

#[derive(Debug)]
pub enum ViewMessage {
    Init,
    Loaded(Box<dyn Any + Send>),
    Selected,
    Unselected,
    //Error(anyhow::Error),
    UpdateBadge(i32),
    Update,
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
}
