use iced::{
    widget::{container, text},
    Command,
};

use crate::iview::{IView, ViewMessage};

#[derive(Default)]
pub struct VolumeView;

impl IView for VolumeView {
    fn view(&self) -> iced::Element<ViewMessage> {
        //button(text("Volume"));
        container(text("Volume")).into()
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => println!("NOT IMPLEMENED Init"),
            ViewMessage::Selected => println!("NOT IMPLEMENED Selected"),
            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Error(err) => println!("{:?}", err),
            ViewMessage::Loaded(_state) => {
                println!("NOT IMPLEMENED Loaded")
                //self.state = *state as State
            }
        }
        Command::none()
    }

    fn get_badge_number(&self) -> Option<i32> {
        None
    }
}
