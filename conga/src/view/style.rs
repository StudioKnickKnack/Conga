use iced::{container, Background, Color};

pub enum Pane {
    Normal,
}

pub enum App {
    Normal,
}

const APP_BG: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x2b as f32 / 255.0,
    0x36 as f32 / 255.0,
);

const APP_TEXT_COLOR: Color = Color::from_rgb(
    0x83 as f32 / 255.0,
    0x94 as f32 / 255.0,
    0x96 as f32 / 255.0,
);

const PANE_BG: Color = Color::from_rgb(
    0x00 as f32 / 255.0,
    0x2b as f32 / 255.0,
    0x36 as f32 / 255.0,
);

impl container::StyleSheet for App {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(APP_BG)),
            text_color: Some(APP_TEXT_COLOR),
            ..Default::default()
        }
    }
}

impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(PANE_BG)),
            ..Default::default()
        }
    }
}
