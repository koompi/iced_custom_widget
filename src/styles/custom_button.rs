use iced::{button, Background, Color};

pub enum CustomButton {
   Default,
   Circular,
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      match self {
         CustomButton::Default => button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 10.0,
            ..button::Style::default()
         },
         CustomButton::Circular => button::Style {
            background: Some(Background::from(Color::from_rgb8(86, 101, 115))),
            border_radius: 25.0,
            text_color: Color::WHITE,
            ..button::Style::default()
         },
      }
   }

   fn hovered(&self) -> button::Style {
      button::Style {
         background: match self {
            CustomButton::Default => Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.2))),
            CustomButton::Circular => Some(Background::Color(Color::from_rgba8(44, 62, 80, 1.0))),
         },
         ..self.active()
      }
   }
}
