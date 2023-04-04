
use iced::{
    widget::{container, text},
    Command,
};

use crate::iview::{IView, ViewMessage};

#[derive(Default)]
pub struct VolumeView;

impl IView for VolumeView {
    fn view<'a>(&self) -> iced::widget::Container<'a, crate::Message, iced::Renderer> {
        //button(text("Volume"));
        container(text("Volume"))
    }

    fn update(&mut self, message: ViewMessage) -> Command<ViewMessage> {
        match message {
            ViewMessage::Init => println!("NOT IMPLEMENED Init"),
            ViewMessage::Selected => println!("NOT IMPLEMENED Selected"),
            ViewMessage::Unselected => println!("NOT IMPLEMENED Unselected"),
            ViewMessage::Loaded(_state) => {
                println!("NOT IMPLEMENED Loaded")
                //self.state = *state as State
            }
        }
        Command::none()
    }
}
