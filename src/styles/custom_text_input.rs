use iced::{text_input, Background, Color};

pub enum CustomTextInput {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
}

impl text_input::StyleSheet for CustomTextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(match self {
                CustomTextInput::Default => Color::from_rgba8(33, 33, 33, 0.2),
                CustomTextInput::Primary => Color::from_rgba8(33, 33, 33, 0.2),
                CustomTextInput::Secondary => Color::from_rgba8(0, 191, 165, 0.2),
                CustomTextInput::Success => Color::from_rgba8(0, 200, 83, 0.2),
                CustomTextInput::Warning => Color::from_rgba8(255, 214, 0, 0.2),
                CustomTextInput::Error => Color::from_rgba8(221, 44, 0, 0.2),
            }),
            border_radius: 12,
            border_color: Color::BLACK,
            border_width: 0,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            border_width: 1,
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb(0.3, 0.3, 0.3)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb(1., 1., 1.)
    }
}
