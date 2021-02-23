use iced::{
    button::{Style, StyleSheet},
    Background, Color, Vector,
};
pub enum ButtonStyle {
    Default,
    Circular(u8, u8, u8, f32),
    BigCircular(u8, u8, u8, f32),
    CircleRadius(u8, u8, u8, f32, f32, Color),
    Transparent,
}

impl StyleSheet for ButtonStyle {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: match self {
                ButtonStyle::Default => Some(Background::Color([0.87, 0.87, 0.87].into())),
                ButtonStyle::Circular(c1, c2, c3, p)
                | ButtonStyle::CircleRadius(c1, c2, c3, p, _, _)
                | ButtonStyle::BigCircular(c1, c2, c3, p) => {
                    Some(Background::Color(Color::from_rgba8(*c1, *c2, *c3, *p)))
                }
                ButtonStyle::Transparent => Some(Background::Color(Color::TRANSPARENT)),
            },
            border_radius: match self {
                ButtonStyle::Default | ButtonStyle::Circular(_, _, _, _) => 4.0,
                ButtonStyle::BigCircular(_, _, _, _) => 25.0,
                ButtonStyle::Transparent => 0.0,
                ButtonStyle::CircleRadius(_, _, _, _, r, _) => *r,
            },
            border_width: 0.0,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: match self {
                ButtonStyle::Default
                | ButtonStyle::BigCircular(_, _, _, _)
                | ButtonStyle::Circular(_, _, _, _) => Color::WHITE,
                ButtonStyle::Transparent => Color::BLACK,
                ButtonStyle::CircleRadius(_, _, _, _, _, color) => *color,
            },
        }
    }
}
