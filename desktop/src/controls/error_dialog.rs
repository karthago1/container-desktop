use iced::{theme, widget::container, Length};

use super::dialog::Dialog;

pub fn error_dialog<'a, Message>(
    err: &str,
    content: iced::Element<'a, Message>,
    close_msg: Message,
) -> iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    let modal: iced::Element<Message> = container(
        iced::widget::column![
            iced::widget::text("Error").size(24),
            iced::widget::column![
                iced::widget::text(err),
                container(
                    iced::widget::button(
                        iced::widget::text("Close")
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                    )
                    .on_press(close_msg.clone())
                )
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Right),
            ]
            .spacing(10)
        ]
        .spacing(20),
    )
    .width(300)
    .padding(10)
    .style(theme::Container::Box)
    .into();

    Dialog::new(content, modal).on_blur(close_msg).into()
}
