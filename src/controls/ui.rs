use iced::{theme, widget};

use crate::style;

pub fn icon_button<'a, Message>(icon: &str) -> iced::widget::Button<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    widget::button(widget::image(format!(
        "{}/icons/{icon}",
        env!("CARGO_MANIFEST_DIR")
    )))
    .style(theme::Button::Custom(Box::<style::Button>::default()))
}

pub fn icon_status<'a, Message>(
    icon: &str,
    active: bool,
) -> iced::widget::Container<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    widget::container(widget::image(format!(
        "{}/icons/{icon}",
        env!("CARGO_MANIFEST_DIR")
    )))
    .style(if active {
        iced::theme::Container::Custom(Box::new(style::ContainerBackground(*style::colors::ACCENT)))
    } else {
        iced::theme::Container::Transparent
    })
}
