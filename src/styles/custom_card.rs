use super::card::{Style, StyleSheet};
use iced::{Background, Color};

pub enum CustomCard {
   Default,
}

impl StyleSheet for CustomCard {
   fn active(&self) -> Style {
      match self {
         CustomCard::Default => Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border_radius: 10.0,
            ..Style::default()
         },
      }
   }

   fn hovered(&self) -> Style {
      Style {
         background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.2))),
         ..self.active()
      }
   }
}
