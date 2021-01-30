use iced::container::{Style, StyleSheet};
use iced_core::{Background, Color};

pub enum ContainerStyle {
    Custom,
    InkColor,
    LightGray,
    White,
    LightGrayCircle,
    Black,
}
impl StyleSheet for ContainerStyle {
    fn style(&self) -> iced::container::Style {
        Style {
            text_color: None,
            background: match self {
                ContainerStyle::Custom => {
                    Some(Background::Color(Color::from_rgba8(223, 228, 234, 1.0)))
                }
                ContainerStyle::InkColor => {
                    Some(Background::from(Color::from_rgba8(206, 214, 224, 1.0)))
                }
                ContainerStyle::LightGray => {
                    Some(Background::from(Color::from_rgba8(215, 219, 221, 1.0)))
                }
                ContainerStyle::White => {
                    Some(Background::from(Color::from_rgba8(255, 255, 255, 1.0)))
                }
                ContainerStyle::LightGrayCircle => {
                    Some(Background::from(Color::from_rgba8(215, 219, 221, 0.5)))
                }
                ContainerStyle::Black => Some(Background::from(Color::BLACK)),
            },
            border_radius: match self {
                ContainerStyle::Custom
                | ContainerStyle::LightGrayCircle
                | ContainerStyle::White
                | ContainerStyle::InkColor
                | ContainerStyle::Black => 10.0,
                ContainerStyle::LightGray => 0.0,
            },
            border_width: 0.0,
            border_color: Color::from_rgba8(255, 255, 255, 1.0),
        }
    }
}
