use iced::{
   button, Color, Background, container, Vector
};

pub enum CustomButton {
   Default,
   Secondary,
   Sidebar,
   Text,
   Selected,
   Tab,
   SelectedTab
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      button::Style {
         text_color: match self {
            CustomButton::Sidebar => Color::from_rgb8(97, 97, 97),
            CustomButton::Selected => Color::from_rgb8(15, 86, 179),
            _ => Color::BLACK
         },
         background: Some(Background::Color(match self {
            CustomButton::Selected | CustomButton::SelectedTab => Color::from_rgba8(15, 86, 179, 0.3),
            CustomButton::Text | CustomButton::Tab => Color::TRANSPARENT,
            _ => Color::WHITE,
         })),
         border_radius: match self {
            CustomButton::Text => 10.0,
            _ => 5.0
         },
         border_color: match self {
            CustomButton::Default | CustomButton::Secondary => Color::BLACK,
            _ => Color::TRANSPARENT
         },
         border_width: match self {
            CustomButton::Secondary => 1.0,
            _ => 0.0
         },
         shadow_offset: match self {
            CustomButton::Default | CustomButton::Secondary => Vector::new(0.5, 1.0),
            _ => Vector::new(0.0, 0.0),
         }
      }
   }

   fn hovered(&self) -> button::Style {
      let active = self.active();

      match self {
         CustomButton::Sidebar | CustomButton::Text | CustomButton::Tab => button::Style {
            background: Some(Background::Color(Color {
               a: 0.2,
               ..active.text_color
            })),
            ..active
         },
         _ => active
      }
   }
}

pub enum CustomContainer {
   Background,
   ForegroundWhite,
   ForegroundGray,
   Header,
   Segment
}

impl container::StyleSheet for CustomContainer {
   fn style(&self) -> container::Style {
      container::Style {
         background: Some(Background::Color(match self {
            CustomContainer::Background => Color::from_rgb8(238, 238, 238),
            CustomContainer::ForegroundWhite => Color::WHITE,
            CustomContainer::ForegroundGray => Color::from_rgb8(224, 224, 224),
            CustomContainer::Segment => Color::TRANSPARENT,
            CustomContainer::Header => Color::from_rgb8(238, 238, 238)
         })),
         border_radius: match self {
            CustomContainer::ForegroundGray | CustomContainer::Segment => 7.0,
            _ => 0.0
         },
         border_width: match self {
            CustomContainer::Header | CustomContainer::Segment => 0.5,
            _ => 0.0
         },
         border_color: match self {
            CustomContainer::Header => Color::TRANSPARENT,
            CustomContainer::Segment => Color::from_rgb8(15, 86, 179),
            _ => Color::from_rgb8(238, 238, 238),
         },
         ..container::Style::default()
      }
   }
}