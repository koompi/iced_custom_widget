use iced::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Themes {
    Light,
    Dark,
}

impl Themes {
    pub const ALL: [Themes; 2] = [Themes::Light, Themes::Dark];

    pub fn background(&self) -> Color {
        match self {
            Themes::Light => Color::from_rgb8(54, 57, 63),
            Themes::Dark => Color::from_rgb8(255, 255, 255),
        }
    }

    pub fn foreground(&self) -> Color {
        match self {
            Themes::Light => Color::from_rgb8(244, 246, 247),
            Themes::Dark => Color::from_rgb8(64, 68, 75),
        }
    }

    pub fn scrollbar(&self) -> Color {
        match self {
            Themes::Light => Color::from_rgb8(253, 254, 254),
            Themes::Dark => Color::from_rgb8(46, 51, 56),
        }
    }

    pub fn scroller(&self) -> Color {
        match self {
            Themes::Light => Color::from_rgb8(202, 207, 210),
            Themes::Dark => Color::from_rgb8(32, 34, 37),
        }
    }
}

impl Default for Themes {
    fn default() -> Self {
        Self::Light
    }
}

impl Display for Themes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Themes::Light => "Light",
                Themes::Dark => "Dark",
            }
        )
    }
}
