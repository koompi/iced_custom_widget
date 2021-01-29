use iced::pick_list::{Menu, Style, StyleSheet};
use iced_core::{Background, Color};
pub struct PickListStyle;

impl StyleSheet for PickListStyle {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgba8(215, 219, 221, 1.0)),
            border_width: 0.5,
            border_color: [0.7, 0.7, 0.7].into(),
            selected_text_color: Color::WHITE,
            selected_background: Background::Color(Color::from_rgba8(86, 101, 115, 1.0)),
        }
    }
    fn active(&self) -> Style {
        Style {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
            border_radius: 10.0,
            border_width: 0.0,
            border_color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            border_color: Color::BLACK,
            ..self.active()
        }
    }
}
