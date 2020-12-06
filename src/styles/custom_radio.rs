use iced::{radio, Background, Color};

pub enum CustomRadio {
    Default,
    Blue,
    Purple,
    Pink,
    BoldPink,
    Orange,
    Yellow,
    Green,
    Gray,
}

impl radio::StyleSheet for CustomRadio {
    fn active(&self) -> radio::Style {
        match self {
            CustomRadio::Default => radio::Style {
                background: Background::Color(Color::from_rgb8(95, 39, 205)),
                dot_color: Color::from_rgb8(87, 101, 116),
                border_width: 2.0,
                border_color: Color::from_rgb8(95, 39, 205),
            },
            CustomRadio::Blue => radio::Style {
                background: Background::Color(Color::from_rgb8(9, 132, 227)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Purple => radio::Style {
                background: Background::Color(Color::from_rgb8(142, 68, 173)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Pink => radio::Style {
                background: Background::Color(Color::from_rgb8(253, 121, 168)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::BoldPink => radio::Style {
                background: Background::Color(Color::from_rgb8(232, 67, 147)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Orange => radio::Style {
                background: Background::Color(Color::from_rgb8(255, 118, 117)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Yellow => radio::Style {
                background: Background::Color(Color::from_rgb8(254, 202, 87)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Green => radio::Style {
                background: Background::Color(Color::from_rgb8(32, 191, 107)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
            CustomRadio::Gray => radio::Style {
                background: Background::Color(Color::from_rgb8(119, 140, 163)),
                dot_color: Color::WHITE,
                border_width: 0.0,
                border_color: Color::from_rgb8(9, 132, 227),
            },
        }
    }
    fn hovered(&self) -> radio::Style {
        radio::Style {
            background: Background::Color(Color::from_rgb8(75, 101, 132)),
            ..self.active()
        }
    }
}
