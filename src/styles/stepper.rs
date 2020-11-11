use iced_native::{Background, Color};

pub struct Style {
   pub text_background: Option<Background>,
   pub button_background: Option<Background>,
   pub border_radius: u16,
   pub border_width: u16,
   pub border_color: Color,
   pub text_color: Color,
}

impl std::default::Default for Style {
   fn default() -> Self {
      Self {
         text_background: None,
         button_background: None,
         border_radius: 2,
         border_width: 1,
         border_color: Color::BLACK,
         text_color: Color::BLACK,
      }
   }
}

pub trait StyleSheet {
   fn active(&self) -> Style;

   fn hovered(&self) -> Style {
      let active = self.active();
      Style {
         button_background: Some(Background::Color(Color {
            a: active.text_color.a * 0.3,
            ..active.text_color
         })),
         ..active
      }
   }

   fn pressed(&self) -> Style {
      self.active()
   }

   fn disabled(&self) -> Style {
      let active = self.active();
      Style {
         button_background: active.button_background.map(|bg| match bg {
            Background::Color(color) => Background::Color(Color {
               a: color.a * 0.5,
               ..color
            })
         }),
         text_color: Color {
            a: active.text_color.a * 0.5,
            ..active.text_color
         },
         ..active
      }
   }
}

struct Default;

impl StyleSheet for Default {
   fn active(&self) -> Style {
      let default = Style::default();
      Style {
         border_color: Color {
            a: default.text_color.a * 0.3,
            ..default.text_color
         },
         ..default
      }
   }
}

impl std::default::Default for Box<dyn StyleSheet> {
   fn default() -> Self {
      Box::new(Default)
   }
}

impl<T> From<T> for Box<dyn StyleSheet> 
where T: 'static + StyleSheet {
   fn from(style: T) -> Self {
      Box::new(style)
   }
}