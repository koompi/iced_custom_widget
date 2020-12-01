use iced::{
   button, Color, Background
};

pub struct SidebarButton;

impl button::StyleSheet for SidebarButton {
   fn active(&self) -> button::Style { 
      button::Style {
         text_color: Color::from_rgb8(97, 97, 97),
         border_radius: 7.0,
         ..button::Style::default()
      }
   }

   fn hovered(&self) -> button::Style {
      let active = self.active();

      button::Style {
         text_color: Color::from_rgb8(15, 86, 179),
         background: Some(Background::Color(Color {
            a: 0.2,
            ..active.text_color
         })),
         ..active
      }
   }

   fn pressed(&self) -> button::Style {
      let hovered = self.hovered();

      button::Style {
         background: Some(Background::Color(Color {
            a: 0.3,
            ..hovered.text_color
         })),
         ..hovered
      }
   }
} 