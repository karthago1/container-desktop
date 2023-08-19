use iced::{
    widget::button::{self, Appearance},
    Background, Color, Theme, Vector,
};

use super::colors;

pub struct Button {
    radius: f32,
    background: Option<Background>,
    text_color: Color,
}

/*impl Button {
    pub fn new(radius: f32, background: Option<Background>, text_color: Color) -> Self {
        Self {
            radius,
            background,
            text_color,
        }
    }
}*/

impl Default for Button {
    fn default() -> Self {
        Self {
            radius: Default::default(),
            background: None,
            text_color: *colors::PRIMARY_TEXT,
        }
    }
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            background: self.background,
            border_radius: self.radius.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: self.text_color,
        }
    }

    /// Produces the hovered [`Appearance`] of a button.
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            border_width: 2.,
            border_color: *colors::ACCENT,
            ..active
        }
    }

    /// Produces the pressed [`Appearance`] of a button.
    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(*colors::ACCENT)),
            text_color: *colors::ACCENT_TEXT,
            ..self.active(style)
        }
    }
}
