use iced::{Background, Color};
use super::card::{StyleSheet, Style};

pub enum CustomCard {
    Default,
}

impl StyleSheet for CustomCard {
    fn active(&self) -> Style {
        match self {
            CustomCard::Default => Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border_radius: 10,
                ..Style::default()
            },
        }
    }

    fn hovered(&self) -> Style {
        Style {
            background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.2))),
            ..self.active()
        }
    }
}
