use super::themes::Themes;
use iced::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colors {
    Blue,
    Brown,
    Gray,
    Green,
    Indigo,
    Orange,
    Pink,
    Purple,
    Red,
    Teal,
    Yellow,
}

impl Colors {
    pub const ALL: [Colors; 11] = [
        Colors::Blue,
        Colors::Brown,
        Colors::Gray,
        Colors::Green,
        Colors::Indigo,
        Colors::Orange,
        Colors::Pink,
        Colors::Purple,
        Colors::Red,
        Colors::Teal,
        Colors::Yellow,
    ];

    pub const WHITE: Color = Color::from_rgb(1., 1., 1.);

    pub const BLACK: Color = Color::from_rgb(0., 0., 0.);

    pub fn blue(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(41, 121, 255),
            Themes::Dark => Color::from_rgb8(68, 138, 255),
        }
    }

    pub fn brown(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(109, 76, 65),
            Themes::Dark => Color::from_rgb8(121, 85, 72),
        }
    }

    pub fn gray(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(117, 117, 117),
            Themes::Dark => Color::from_rgb8(158, 158, 158),
        }
    }

    pub fn green(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(0, 200, 83),
            Themes::Dark => Color::from_rgb8(0, 230, 118),
        }
    }

    pub fn indigo(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(63, 81, 181),
            Themes::Dark => Color::from_rgb8(92, 107, 192),
        }
    }

    pub fn orange(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(255, 145, 0),
            Themes::Dark => Color::from_rgb8(255, 171, 64),
        }
    }

    pub fn pink(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(233, 30, 99),
            Themes::Dark => Color::from_rgb8(236, 64, 122),
        }
    }

    pub fn purple(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(170, 0, 255),
            Themes::Dark => Color::from_rgb8(213, 0, 249),
        }
    }

    pub fn red(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(244, 67, 54),
            Themes::Dark => Color::from_rgb8(239, 83, 80),
        }
    }

    pub fn teal(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(0, 150, 136),
            Themes::Dark => Color::from_rgb8(38, 166, 154),
        }
    }

    pub fn yellow(theme: Themes) -> Color {
        match theme {
            Themes::Light => Color::from_rgb8(255, 235, 59),
            Themes::Dark => Color::from_rgb8(255, 235, 59),
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::Blue
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Colors::Blue => "Blue",
                Colors::Brown => "Brown",
                Colors::Gray => "Gray",
                Colors::Green => "Green",
                Colors::Indigo => "Indigo",
                Colors::Orange => "Orange",
                Colors::Pink => "Pink",
                Colors::Purple => "Purple",
                Colors::Red => "Red",
                Colors::Teal => "Teal",
                Colors::Yellow => "Yellow",
            }
        )
    }
}
