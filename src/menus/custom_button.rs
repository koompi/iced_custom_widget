use iced::{
    button, Background, Color
};

pub enum CustomButton {
    Default
}

impl button::StyleSheet for CustomButton {
    fn active(&self) -> button::Style {
        match self {
            CustomButton::Default => {
                button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border_radius: 10,
                    ..button::Style::default()
                }
            }
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.2))),
            ..self.active()
        }
    }
}