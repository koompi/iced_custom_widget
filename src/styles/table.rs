use iced_native::{Background, Color};

#[derive(Debug, Clone, Copy)]
pub struct Style {
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
         text_color: Color::BLACK,
         background: Background::Color(Color::from_rgb8(245, 245, 245)),
         header_background: Background::Color(Color::TRANSPARENT),
         border_radius: 4.0,
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
         header_background: Background::Color(Color::from_rgba8(52, 152, 219, 0.5)),
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
