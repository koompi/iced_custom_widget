use iced::container::{Style, StyleSheet};
use iced_core::{Background, Color};

pub enum ContainerStyle {
    Custom,
    InkColor,
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
                    Some(Background::from(Color::from_rgba8(223, 228, 234, 1.0)))
                }
            },
            border_radius: 10.0,
            border_width: 0.0,
            border_color: Color::from_rgba8(255, 255, 255, 1.0),
        }
    }
}
