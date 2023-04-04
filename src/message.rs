use crate::{iview::ViewMessage, main_menu::MainMenuMessage};

#[derive(Debug)]
pub enum Message {
    MenuMessage(MainMenuMessage),
    View(IndexedViewMessage),
}

#[derive(Debug)]
pub struct IndexedViewMessage {
    pub index: usize,
    pub msg: ViewMessage,
}

impl IndexedViewMessage {
    pub fn new(index: usize, msg: ViewMessage) -> Self {
        IndexedViewMessage { index, msg }
    }
}
