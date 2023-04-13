use iced::{theme, widget};

use crate::style;

pub fn icon_button<'a, Message>(icon: usize) -> iced::widget::Button<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    let handle = super::icons::icon(icon);
    widget::button(widget::image(handle.clone()))
        .style(theme::Button::Custom(Box::<style::Button>::default()))
}

pub fn icon_status<'a, Message>(
    icon: usize,
    active: bool,
) -> iced::widget::Container<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    let handle = super::icons::icon(icon);
    widget::container(widget::image(handle.clone())).style(if active {
        iced::theme::Container::Custom(Box::new(style::ContainerBackground(*style::colors::ACCENT)))
    } else {
        iced::theme::Container::Transparent
    })
}
