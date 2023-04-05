use container_view::ContainerView;
use iced::{theme, widget::row, Application, Color, Command, Element, Settings, Theme};
use iview::{IView, ViewMessage};
use main_menu::{MainMenu, MainMenuItem};
use message::{IndexedViewMessage, Message};
use volume_view::VolumeView;

#[path = "./views/container_view.rs"]
mod container_view;
#[path = "./views/iview.rs"]
mod iview;
#[path = "./views/volume_view.rs"]
mod volume_view;

mod controls;

mod main_menu;

mod message;

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
        let mut w = MainWindow {
            menu: MainMenu::new(vec![
                MainMenuItem::new("Containers".to_string(), "container.png".to_string()),
                MainMenuItem::new("Images".to_string(), "image.png".to_string()),
                MainMenuItem::new("Volumes".to_string(), "settings.png".to_string()),
                MainMenuItem::new("Settings".to_string(), "settings.png".to_string()),
            ]),
            views: vec![
                Box::<ContainerView>::default(),
                Box::<ContainerView>::default(),
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
            background: Color::from_rgb(0x1b as f32 / 255., 0x1b as f32 / 255., 0x1b as f32 / 255.),
            text: Color::WHITE,
            primary: Color::from_rgb(0x42 as f32 / 255., 0x42 as f32 / 255., 0x42 as f32 / 255.),
            success: Color::from_rgb(1.0, 0x6f as f32 / 255., 0.),
            danger: Color::from_rgb(0xef as f32 / 255., 0x53 as f32 / 255., 0x50 as f32 / 255.),
        })
    }
}
