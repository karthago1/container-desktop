use iced::Color;

pub static PRIMARY: &Color = &Color {
    r: 0x42 as f32 / 255.,
    g: 0x42 as f32 / 255.,
    b: 0x42 as f32 / 255.,
    a: 1.,
};

pub static PRIMARY_DARK: &Color = &Color {
    r: 0x1b as f32 / 255.,
    g: 0x1b as f32 / 255.,
    b: 0x1b as f32 / 255.,
    a: 1.,
};

pub static PRIMARY_LIGHT: &Color = &Color {
    r: 0x6d as f32 / 255.,
    g: 0x6d as f32 / 255.,
    b: 0x6d as f32 / 255.,
    a: 1.,
};

pub static PRIMARY_TEXT: &Color = &Color {
    r: 1.,
    g: 1.,
    b: 1.,
    a: 1.,
};

pub static ACCENT: &Color = &Color {
    r: 1.,
    g: 0x6f as f32 / 255.,
    b: 0.,
    a: 1.,
};

pub static ACCENT_TEXT: &Color = &Color {
    r: 1.,
    g: 1.,
    b: 1.,
    a: 1.,
};

pub static ERROR: &Color = &Color {
    r: 0xef as f32 / 255.,
    g: 0x53 as f32 / 255.,
    b: 0x50 as f32 / 255.,
    a: 1.,
};
