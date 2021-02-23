use iced::progress_bar::{Style, StyleSheet};
use iced_core::{Background, Color};
pub enum SliderStyle {
    Default,
    Circle(u8, u8, u8, f32, f32),
    BigCircle(u8, u8, u8, f32, f32),
    WhiteGrayCircle(u8, u8, u8, f32, f32),
}

impl StyleSheet for SliderStyle {
    fn style(&self) -> Style {
        Style {
            background: Background::Color(Color::from_rgb(0.6, 0.6, 0.6)),
            bar: match self {
                SliderStyle::WhiteGrayCircle(r, b, g, alpha, _)
                | SliderStyle::Circle(r, b, g, alpha, _)
                | SliderStyle::BigCircle(r, b, g, alpha, _) => {
                    Background::Color(Color::from_rgba8(*r, *b, *g, *alpha))
                }
                SliderStyle::Default => Background::Color(Color::from_rgb(0.3, 0.9, 0.3)),
            },
            border_radius: match self {
                SliderStyle::WhiteGrayCircle(_, _, _, _, r)
                | SliderStyle::BigCircle(_, _, _, _, r)
                | SliderStyle::Circle(_, _, _, _, r) => *r,
                SliderStyle::Default => 5.0,
            },
        }
    }
}
