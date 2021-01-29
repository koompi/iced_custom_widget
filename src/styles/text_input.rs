use iced::text_input::{Style, StyleSheet};
use iced_core::{Background, Color};
pub enum InputStyle {
    Default,
    CircularBorder,
    InkBorder,
}

impl StyleSheet for InputStyle {
    fn active(&self) -> Style {
        Style {
            background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
            border_radius: 8.0,
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
        }
    }

    fn focused(&self) -> Style {
        Style {
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            background: Background::from(Color::from_rgba8(215, 219, 221, 0.5)),
            border_width: match self {
                InputStyle::Default => 1.0,
                InputStyle::CircularBorder | InputStyle::InkBorder => 2.0,
            },
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }

    fn value_color(&self) -> Color {
        Color::from_rgba8(86, 101, 115, 1.0)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 1.0)
    }
}
