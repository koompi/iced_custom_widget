use iced::{
   button, Color, Background
};

pub enum CustomButton {
   Default,
   Sidebar,
   Selected
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      button::Style {
         text_color: match self {
            CustomButton::Default => Color::BLACK,
            CustomButton::Sidebar => Color::from_rgb8(97, 97, 97),
            CustomButton::Selected => Color::from_rgb8(15, 86, 179)
         },
         background: Some(Background::Color(match self {
            CustomButton::Selected => Color::from_rgba8(15, 86, 179, 0.3),
            _ => Color::TRANSPARENT
         })),
         border_radius: 10.0,
         ..button::Style::default()
      }
   }

   fn hovered(&self) -> button::Style {
      let active = self.active();

      match self {
         CustomButton::Default => button::Style {
            background: Some(Background::Color(Color::from_rgba8(0, 0, 0, 0.2))),
            ..active
         },
         CustomButton::Sidebar => button::Style {
            text_color: Color::from_rgb8(15, 86, 179),
            background: Some(Background::Color(Color {
               a: 0.2,
               ..active.text_color
            })),
            ..active
         },
         CustomButton::Selected => active
      }
   }
}