use provider::Provider;

use container_view::ContainerView;
use iced::{theme, widget::row, Application, Command, Element, Settings, Theme};
use image_view::ImageView;
use iview::{IView, ViewMessage};
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
mod main_menu;
mod message;
mod provider;
mod style;

fn main() -> iced::Result {
    MainWindow::run(Settings::default())
}

struct MainWindow {
    menu: MainMenu,
    views: Vec<Box<dyn IView>>,
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (MainWindow, Command<Message>) {
        Provider::initialize();

        let mut w = MainWindow {
            menu: MainMenu::new(vec![
                MainMenuItem::new("Containers".to_string(), "container.png".to_string()),
                MainMenuItem::new("Images".to_string(), "image.png".to_string()),
                MainMenuItem::new("Volumes".to_string(), "settings.png".to_string()),
                MainMenuItem::new("Settings".to_string(), "settings.png".to_string()),
            ]),
            views: vec![
                Box::<ContainerView>::default(),
                Box::<ImageView>::default(),
                Box::<VolumeView>::default(),
                Box::<VolumeView>::default(),
            ],
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
            Message::View(msg) => {
                if let ViewMessage::UpdateBadge(badge) = msg.msg {
                    self.menu
                        .update(main_menu::MainMenuMessage::Badge(msg.index, Some(badge)));
                    Command::none()
                } else {
                    self.views[msg.index].update(msg.msg).map(move |new_msg| {
                        Message::View(IndexedViewMessage::new(msg.index, new_msg))
                    })
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row(vec![
            self.menu.view().map(Message::MenuMessage),
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
