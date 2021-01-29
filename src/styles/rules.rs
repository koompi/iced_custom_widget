use iced::rule::{FillMode, Style, StyleSheet};
use iced_core::Color;
pub struct RuleStyle {}

impl StyleSheet for RuleStyle {
    fn style(&self) -> Style {
        Style {
            color: Color::WHITE,
            width: 1,
            radius: 0.0,
            fill_mode: FillMode::Percent(100.0),
        }
    }
}
