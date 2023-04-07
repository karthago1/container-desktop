use iced::{widget::container, Background, Color, Theme};

use super::colors;

pub struct ContainerBackground(pub Color);

impl Default for ContainerBackground {
    fn default() -> Self {
        Self(*colors::PRIMARY_DARK)
    }
}

impl container::StyleSheet for ContainerBackground {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: Some(Background::Color(self.0)),
            border_radius: 5.,
            border_width: 0.,
            border_color: Color::TRANSPARENT,
        }
    }
}
