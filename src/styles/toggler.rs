//! Show toggle controls using togglers.
use iced_core::Color;

/// The appearance of a toggler.
#[derive(Debug)]
pub struct Style {
    pub background: Color,
    pub background_border: Option<Color>,
    pub foreground: Color,
    pub foreground_border: Option<Color>,
}
/// A set of rules that dictate the style of a toggler.
pub trait StyleSheet {
    fn active(&self, is_active: bool) -> Style;

    fn hovered(&self, is_active: bool) -> Style;
}

enum ToggleStyle {
    Default,
    SelectStyle
}

impl StyleSheet for ToggleStyle {
    fn active(&self, is_active: bool) -> Style {
        Style {
            background: if is_active {
                Color::from_rgb(0.0, 1.0, 0.0)
            } else {
                Color::from_rgb(0.7, 0.7, 0.7)
                // Color::BLACK
            },
            background_border: None,
            foreground: Color::WHITE,
            foreground_border: None,
        }
    }

    fn hovered(&self, is_active: bool) -> Style {
        Style {
            foreground: Color::from_rgb(0.95, 0.95, 0.95),
            ..self.active(is_active)
        }
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(ToggleStyle::Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}
