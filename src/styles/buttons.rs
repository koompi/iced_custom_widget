use iced::{
    button::{Style, StyleSheet},
    Background, Color, Vector,
};
pub enum ButtonStyle {
    Default,
    Circular,
    BigCircular,
}

impl StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: match self {
                ButtonStyle::Default => Some(Background::Color([0.87, 0.87, 0.87].into())),
                ButtonStyle::Circular | ButtonStyle::BigCircular => {
                    Some(Background::Color(Color::from_rgba8(86, 101, 115, 1.0)))
                }
            },
            border_radius: match self {
                ButtonStyle::Default | ButtonStyle::Circular => 4.0,
                ButtonStyle::BigCircular => 25.0,
            },
            border_width: 0.0,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: Color::WHITE,
        }
    }
}
