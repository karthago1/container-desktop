use crate::{main_menu::MainMenuMessage, views::ViewMessage};

#[derive(Debug)]
pub enum Message {
    MenuMessage(MainMenuMessage),
    View(IndexedViewMessage),
    Update,
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
