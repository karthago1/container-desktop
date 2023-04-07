use core::CorePlugin;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use container_view::ContainerView;
use iced::{theme, widget::row, Application, Command, Element, Settings, Theme};
use image_view::ImageView;
use iview::{IView, ViewMessage};
use libloading::Library;
use main_menu::{MainMenu, MainMenuItem};
use message::{IndexedViewMessage, Message};
use style::colors;
use volume_view::VolumeView;

#[path = "./views/iview.rs"]
mod iview;

#[path = "./views/container_view.rs"]
mod container_view;

#[path = "./views/image_view.rs"]
mod image_view;

#[path = "./views/volume_view.rs"]
mod volume_view;

mod controls;
mod style;

mod main_menu;

mod message;

static PLUGIN_ENTRY_FUNCTION: &[u8] = b"initialize\0";

fn main() -> iced::Result {
    MainWindow::run(Settings::default())
}

struct MainWindow {
    menu: MainMenu,
    views: Vec<Box<dyn IView>>,
    _libs: Vec<Library>,
}

fn get_plugin(dir: &Path) -> PathBuf {
    fs::read_dir(dir)
        .unwrap()
        .find(|f| match f {
            Ok(f) => {
                f.file_type().unwrap().is_file()
                    && f.file_name().to_str().unwrap().ends_with("_client.so")
            }
            Err(_) => false,
        })
        .map(|f| f.unwrap().path())
        .unwrap()
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (MainWindow, Command<Message>) {
        let exe = std::env::current_exe().unwrap();
        let exe_dir = exe.parent().unwrap();

        println!("scanning {:?}", exe_dir);
        let lib_path = get_plugin(exe_dir);

        let mut libs = Vec::<Library>::new();
        let plugin = unsafe {
            let lib = libloading::Library::new(lib_path).unwrap();

            let res = {
                let plugin_entry_fn: libloading::Symbol<unsafe extern "C" fn() -> CorePlugin> =
                    lib.get(PLUGIN_ENTRY_FUNCTION).unwrap();
                plugin_entry_fn()
            };

            libs.push(lib);

            Arc::new(Mutex::new(Box::new(res)))
        };

        let mut w = MainWindow {
            menu: MainMenu::new(vec![
                MainMenuItem::new("Containers".to_string(), "container.png".to_string()),
                MainMenuItem::new("Images".to_string(), "image.png".to_string()),
                MainMenuItem::new("Volumes".to_string(), "settings.png".to_string()),
                MainMenuItem::new("Settings".to_string(), "settings.png".to_string()),
            ]),
            views: vec![
                Box::<ContainerView>::default(),
                Box::new(ImageView::new(plugin)),
                Box::<VolumeView>::default(),
                Box::<VolumeView>::default(),
            ],
            _libs: libs,
        };

        let cmd = w
            .views
            .first_mut()
            .unwrap()
            .update(ViewMessage::Init)
            .map(|msg| Message::View(IndexedViewMessage::new(0, msg)));

        (w, cmd)
    }

    fn title(&self) -> String {
        "Container Desktop".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuMessage(message) => {
                let _ = self.views[self.menu.selected_index].update(ViewMessage::Unselected);
                self.menu.update(message);
                let view_index = self.menu.selected_index;
                self.views[view_index]
                    .update(ViewMessage::Selected)
                    .map(move |new_msg| Message::View(IndexedViewMessage::new(view_index, new_msg)))
            }
            Message::View(msg) => self.views[msg.index]
                .update(msg.msg)
                .map(move |new_msg| Message::View(IndexedViewMessage::new(msg.index, new_msg))),
        }
    }

    fn view(&self) -> Element<Message> {
        let badges: Vec<Option<i32>> = self.views.iter().map(|e| e.get_badge_number()).collect();
        row(vec![
            self.menu.view(badges).map(Message::MenuMessage),
            self.views[self.menu.selected_index]
                .view()
                .map(|msg| Message::View(IndexedViewMessage::new(self.menu.selected_index, msg))),
        ])
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(theme::Palette {
            background: *colors::PRIMARY_DARK,
            text: *colors::PRIMARY_TEXT,
            primary: *colors::PRIMARY,
            success: *colors::ACCENT,
            danger: *colors::ERROR,
        })
    }
}