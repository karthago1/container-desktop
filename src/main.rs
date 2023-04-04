use container_view::ContainerView;
use iced::{theme, widget::row, Application, Color, Command, Element, Length, Settings, Theme};
use iview::{IView, ViewMessage};
use main_menu::{MainMenu, MainMenuItem};
use message::Message;
use volume_view::VolumeView;

#[path = "./views/container_view.rs"]
mod container_view;
#[path = "./views/iview.rs"]
mod iview;
#[path = "./views/volume_view.rs"]
mod volume_view;

#[path = "./views/controls/container_item.rs"]
mod container_item;

#[path = "./views/controls/button_chip_style.rs"]
mod button_chip_style;

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
                MainMenuItem {
                    text: "Dashboard".to_string(),
                    icon: "ferris.png".to_string(),
                },
                MainMenuItem {
                    text: "Containers".to_string(),
                    icon: "container.png".to_string(),
                },
                MainMenuItem {
                    text: "Images".to_string(),
                    icon: "image.png".to_string(),
                },
                MainMenuItem {
                    text: "Volumes".to_string(),
                    icon: "settings.png".to_string(),
                },
                MainMenuItem {
                    text: "Settings".to_string(),
                    icon: "settings.png".to_string(),
                },
            ]),
            views: vec![
                Box::new(ContainerView::default()),
                Box::new(ContainerView::default()),
                Box::new(ContainerView::default()),
                Box::new(VolumeView::default()),
                Box::new(VolumeView::default()),
            ],
        };

        let cmd = w
            .views
            .first_mut()
            .unwrap()
            .update(ViewMessage::Init)
            .map(|c| Message::View(c));

        (w, cmd)
    }

    fn title(&self) -> String {
        "VM Desktop".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuMessage(message) => {
                let _ = self.views[self.menu.selected_index].update(ViewMessage::Unselected);
                self.menu.update(message);
                self.views[self.menu.selected_index]
                    .update(ViewMessage::Selected)
                    .map(|c| Message::View(c))
            }
            Message::View(message) => self.views[self.menu.selected_index]
                .update(message)
                .map(|c| Message::View(c)),
        }
    }

    fn view(&self) -> Element<Message> {
        row(vec![
            self.menu.view().map(move |m| Message::MenuMessage(m)),
            self.views[self.menu.selected_index]
                .view()
                .height(Length::Fill)
                .into(),
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
