use crate::{iview::ViewMessage, main_menu::MainMenuMessage};

#[derive(Debug)]
pub enum Message {
    MenuMessage(MainMenuMessage),
    View(ViewMessage),
}