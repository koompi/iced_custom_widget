use iced_native::{Background, Color};
use super::table_row;

pub struct Style {
   pub table_row: table_row::Style,
   pub text_color: Color,
   pub background: Background,
   pub header_background: Background,
   pub border_radius: f32,
   pub border_width: f32,
   pub border_color: Color,
   pub icon_size: f32,
}

impl std::default::Default for Style {
   fn default() -> Self {
      Self {
         table_row: table_row::Style::default(),
         text_color: Color::BLACK,
         background: Background::Color(Color::WHITE),
         header_background: Background::Color(Color::from_rgba8(238, 238, 238, 1.0)),
         border_radius: 8.0,
         border_width: 1.0,
         border_color: Color::BLACK,
         icon_size: 0.7,
      }
   }
}

pub trait StyleSheet {
   fn active(&self) -> Style;

   fn header_hoverd(&self) -> Style {
      let active = self.active();

      Style {
         header_background: match active.header_background {
            Background::Color(color) => Background::Color(Color {
               a: 1.0,
               ..color
            })
         },
         ..active
      }
   }
}

pub struct Default;

impl StyleSheet for Default {
   fn active(&self) -> Style {
      Style::default()
   }
}

impl std::default::Default for Box<dyn StyleSheet> {
   fn default() -> Self {
      Box::new(Default)
   }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
   T: 'static + StyleSheet,
{
   fn from(style: T) -> Self {
      Box::new(style)
   }
}
