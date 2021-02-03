use iced::slider::{Handle, HandleShape, Style, StyleSheet};
use iced_core::Color;

pub enum SliderStyle {
    Default,
    Circle,
}

impl StyleSheet for SliderStyle {
    fn active(&self) -> Style {
        Style {
            rail_colors: (
                Color::from_rgba8(128, 139, 150, 1.0),
                Color::from_rgba8(128, 139, 150, 1.0),
            ),
            handle: Handle {
                shape: match self {
                    SliderStyle::Default => HandleShape::Rectangle {
                        width: 24,
                        border_radius: 8.0,
                    },
                    SliderStyle::Circle => HandleShape::Circle { radius: 12.0 },
                },
                color: Color::from_rgba8(128, 139, 150, 1.5),
                border_color: Color::from_rgba8(44, 62, 80, 1.0),
                border_width: 1.0,
            },
        }
    }
    fn hovered(&self) -> Style {
        let active = self.active();
        Style {
            handle: Handle {
                color: Color::from_rgba8(205, 213, 203, 1.0),
                ..active.handle
            },
            ..active
        }
    }
    fn dragging(&self) -> Style {
        let active = self.active();

        Style {
            handle: Handle {
                color: Color::from_rgba8(205, 213, 203, 1.0),
                ..active.handle
            },
            ..active
        }
    }
}
