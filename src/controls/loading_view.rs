use iced::{
    theme,
    widget::{container, text},
    Length,
};

pub fn loading_view<'a, Message>() -> iced::widget::Container<'a, Message, iced::Renderer>
where
    Message: 'a,
{
    container(text("Loading..."))
        .style(theme::Container::Box)
        .width(Length::Fill)
        .center_x()
        .center_y()
}
