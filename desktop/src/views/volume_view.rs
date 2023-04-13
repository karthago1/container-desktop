use iced::{
    widget::{container, text},
    Command,
};

use super::{IView, ViewMessage};

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
            ViewMessage::Update => println!("NOT IMPLEMENED Update"),
            //ViewMessage::Error(err) => println!("{:?}", err),
            ViewMessage::Loaded(_state) => {
                println!("NOT IMPLEMENED Loaded")
                //self.state = *state as State
            }
            ViewMessage::UpdateBadge(_) => (),
        }
        Command::none()
    }
}
