use iced_native::{Background, Color};

pub struct Style {
   pub background: Option<Background>,
   pub selected_color: Option<Color>,
   pub text_color: Color,
   pub border_radius: u16,
}

impl std::default::Default for Style {
   fn default() -> Self {
      Self {
         background: None,
         selected_color: None,
         text_color: Color::BLACK,
         border_radius: 4,
      }
   }
}

pub trait StyleSheet {
   fn active(&self) -> Style;

   fn pressed(&self) -> Style {
      let active = self.active();
      let selected_color = match active.selected_color {
         Some(selected_color) => selected_color,
         None => active.text_color,
      };
      Style {
         background: Some(Background::Color(Color {
            a: selected_color.a * 0.3,
            ..selected_color
         })),
         ..self.active()
      }
   }

   fn selected(&self) -> Style {
      let active = self.active();

      Style {
         background: Some(Background::Color(Color {
            a: active.text_color.a * 0.3,
            ..active.text_color
         })),
         ..active
      }
   }
}

struct Default;

impl StyleSheet for Default {
   fn active(&self) -> Style {
      Style {
         selected_color: Some(Color::from_rgb8(30, 136, 229)),
         background: Some(Background::Color(Color::TRANSPARENT)),
         ..Style::default()
      }
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

impl std::default::Default for Box<dyn StyleSheet> {
   fn default() -> Self {
      Box::new(Default)
   }
}
