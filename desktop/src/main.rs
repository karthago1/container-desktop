use controls::icons;
use iced::{theme, widget::row, Application, Command, Element, Settings, Subscription, Theme};
use main_menu::{MainMenu, MainMenuItem};
use message::{IndexedViewMessage, Message};
use provider::Provider;
use style::colors;
use views::{container_view::ContainerView, volume_view::VolumeView, IView, ViewMessage};

mod controls;
mod main_menu;
mod message;
mod style;
mod views;

#[cfg(not(target_arch = "wasm32"))]
mod provider;

#[cfg(target_arch = "wasm32")]
#[path = "./provider_wasm32.rs"]
mod provider;

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
        icons::load_icons();
        Provider::initialize();

        let mut menus: Vec<MainMenuItem> = Provider::providers()
            .iter()
            .map(|p| MainMenuItem::new(p.get_name().to_string(), icons::ICON_CONTAINER))
            .collect();

        let mut views: Vec<Box<dyn IView>> = (0..Provider::providers().len())
            .map(|i| Box::new(ContainerView::new(i)) as Box<dyn IView>)
            .collect();

        menus.push(MainMenuItem::new(
            "Volumes".to_string(),
            icons::ICON_SETTINGS,
        ));
        menus.push(MainMenuItem::new(
            "Settings".to_string(),
            icons::ICON_SETTINGS,
        ));

        views.push(Box::<VolumeView>::default());
        views.push(Box::<VolumeView>::default());

        let mut w = MainWindow {
            menu: MainMenu::new(menus),
            views,
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
            Message::Update => {
                let msg = ViewMessage::Update;
                let view_index = self.menu.selected_index;
                self.views[view_index]
                    .update(msg)
                    .map(move |new_msg| Message::View(IndexedViewMessage::new(view_index, new_msg)))
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

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(1500)).map(|_| Message::Update)
    }
}
