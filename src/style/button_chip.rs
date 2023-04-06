use iced::{
    widget::button::{self, Appearance},
    Background, Color, Theme, Vector,
};

pub struct ButtonChip {
    radius: f32,
}

impl ButtonChip {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl button::StyleSheet for ButtonChip {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            background: Some(Background::Color(style.palette().success)),
            border_radius: self.radius,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }

    /// Produces the hovered [`Appearance`] of a button.
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }

    /// Produces the pressed [`Appearance`] of a button.
    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            ..self.active(style)
        }
    }

    /// Produces the disabled [`Appearance`] of a button.
    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
}
