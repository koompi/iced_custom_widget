use crate::styles::number_input::StyleSheet;
use std::fmt::Display;
use std::str::FromStr;
use iced_graphics::Primitive;
use iced_native::{
   container, event::{self, Event}, mouse, text_input::{self, Value, cursor}, column, row, keyboard, layout::{Limits, Node}, 
   Align, Background, Clipboard, Color, Container, Element, Hasher, TextInput, Size, Column,
   HorizontalAlignment, Layout, Length, Point, Rectangle, Text, VerticalAlignment, Widget, Row,
};
use num_traits::{Num, NumAssignOps};

pub struct NumberInput<'a, T, Message, Renderer: self::Renderer> {
   state: &'a mut ModifierState,
   value: T,
   step: T,
   bound: (T, T),
   padding: u16,
   size: Option<u16>,
   content: TextInput<'a, Message, Renderer>,
   on_change: Box<dyn Fn(T) -> Message>,
   style: <Renderer as self::Renderer>::Style,
   font: Renderer::Font,
}

impl<'a, T, Message, Renderer> NumberInput<'a, T, Message, Renderer>
where
   T: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
   Message: Clone,
   Renderer: self::Renderer,
{
   pub fn new<F>(state: &'a mut State, value: T, max: T, on_changed: F) -> Self
   where
      F: 'static + Fn(T) -> Message + Copy,
      T: 'static
   {
      let State {input_state, mod_state} = state;
      let padding = <Renderer as self::Renderer>::DEFAULT_PADDING;
      let convert_to_num = move |s: String| on_changed(
         T::from_str(&s).unwrap_or(if s.is_empty() {T::zero()} else {value})
      );

      Self {
         state: mod_state,
         value,
         step: T::one(),
         bound: (T::zero(), max),
         padding,
         size: None,
         content: TextInput::new(input_state, "", format!("{}", value).as_str(), convert_to_num).padding(padding).width(Length::Units(127)),
         on_change: Box::new(on_changed),
         style: <Renderer as self::Renderer>::Style::default(),
         font: Default::default(),
      }
   }

   pub fn step(mut self, step: T) -> Self {
      self.step = step;
      self
   }

   pub fn min(mut self, min: T) -> Self {
      if min < self.bound.1 {
         self.bound.0 = min;
      }
      self
   }

   pub fn max(mut self, max: T) -> Self {
      if max > self.bound.0 {
         self.bound.1 = max;
      }
      self
   }

   pub fn bound(mut self, bound: (T, T)) -> Self {
      if bound.0 < bound.1 {
         self.bound = bound;
      }
      self
   }

   pub fn font(mut self, font: Renderer::Font) -> Self {
      self.font = font;
      self.content = self.content.font(font);
      self
   }

   pub fn width(mut self, width: Length) -> Self {
      self.content = self.content.width(width);
      self
   }

   pub fn max_width(mut self, max_width: u32) -> Self {
      self.content = self.content.max_width(max_width);
      self
   }

   pub fn padding(mut self, units: u16) -> Self {
      self.padding = units;
      self.content = self.content.padding(units);
      self
   }

   pub fn size(mut self, size: u16) -> Self {
      self.size = Some(size);
      self.content = self.content.size(size);
      self
   }

   pub fn on_submit(mut self, message: Message) -> Self {
      self.content = self.content.on_submit(message);
      self
   }

   pub fn style(mut self, style: impl Into<<Renderer as self::Renderer>::Style>) -> Self {
      self.style = style.into();
      self
   }

   pub fn input_style(mut self, style: impl Into<<Renderer as iced_native::text_input::Renderer>::Style>) -> Self {
      self.content = self.content.style(style.into());
      self
   }

   fn decrease_val(&mut self, messages: &mut Vec<Message>) {
      if self.value > self.bound.0 {
         let new_val = self.value - self.step;
         self.value = if new_val > self.bound.0 {
            new_val
         } else {
            self.bound.0
         };
         messages.push((self.on_change)(self.value));
      }
   }

   fn increase_val(&mut self, messages: &mut Vec<Message>) {
      if self.value < self.bound.1 {
         let new_val = self.value + self.step;
         self.value = if new_val < self.bound.1 {
            new_val
         } else {
            self.bound.1
         };
         messages.push((self.on_change)(self.value));
      }
   }
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for NumberInput<'a, T, Message, Renderer>
where
   T: Num + NumAssignOps + PartialOrd + Display + FromStr + ToString + Copy,
   Message: Clone,
   Renderer: self::Renderer + container::Renderer + column::Renderer + row::Renderer,
{
   fn width(&self) -> Length {
      Widget::<Message, Renderer>::width(&self.content)
   }

   fn height(&self) -> Length {
      Length::Shrink
   }

   fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
      let padding = f32::from(self.padding);
      let limits = limits.width(self.width()).height(Length::Shrink).pad(padding);
      let content = self.content.layout(renderer, &limits.loose());
      let txt_size = self.size.unwrap_or(renderer.default_size());
      let icon_size = txt_size*3/4;
      let btn_mod = |c| Container::<(), Renderer>::new(Text::new(format!(" {} ", c)).size(icon_size)).center_y().center_x();
      let mut modifier = if self.padding < Renderer::DEFAULT_PADDING {
         Row::<(), Renderer>::new().spacing(1).width(Length::Shrink).push(btn_mod('+')).push(btn_mod('-')).layout(renderer, &limits.loose())
      } else {
         Column::<(), Renderer>::new().spacing(1).width(Length::Shrink).push(btn_mod('▲')).push(btn_mod('▼')).layout(renderer, &limits.loose())
      };
      let intrinsic = Size::new(
         content.size().width - 3.0,
         content.size().height.max(modifier.size().height)
      );
      modifier.align(Align::End, Align::Center, intrinsic);
      let size = limits.resolve(intrinsic);
      Node::with_children(size, vec![content, modifier])
   }

   fn draw(&self, renderer: &mut Renderer, _defaults: &Renderer::Defaults, layout: Layout<'_>, 
      cursor_position: Point, _viewport: &Rectangle) -> Renderer::Output 
   {
      let bounds = layout.bounds();
      let mut children = layout.children();
      let content_layout = children.next().unwrap();
      let mut mod_children = children.next().unwrap().children();
      let inc_bounds = mod_children.next().unwrap().bounds();
      let dec_bounds = mod_children.next().unwrap().bounds();
      let is_mouse_over = bounds.contains(cursor_position);
      let content = self.content.draw(renderer, content_layout, cursor_position, None);
      let is_decrease_disabled = self.value <= self.bound.0;
      let is_increase_disabled = self.value >= self.bound.1;

      self::Renderer::draw(
         renderer, cursor_position, &self.state, inc_bounds, dec_bounds, is_mouse_over, is_decrease_disabled, 
         is_increase_disabled, content, &self.style, self.font
      )
   }

   fn hash_layout(&self, state: &mut Hasher) {
      use std::hash::Hash;
      struct Marker;
      std::any::TypeId::of::<Marker>().hash(state);

      self.padding.hash(state);
      self.size.hash(state);
      self.content.hash_layout(state);
   }

   fn on_event(&mut self, event: Event, layout: Layout<'_>, cursor_position: Point, messages: &mut Vec<Message>,
      renderer: &Renderer, clipboard: Option<&dyn Clipboard>) -> event::Status 
   {
      let mut children = layout.children();
      let content = children.next().unwrap();
      let mut mod_children = children.next().unwrap().children();
      let inc_bounds = mod_children.next().unwrap().bounds();
      let dec_bounds = mod_children.next().unwrap().bounds();
      let mouse_over_inc = inc_bounds.contains(cursor_position);
      let mouse_over_dec = dec_bounds.contains(cursor_position);

      if layout.bounds().contains(cursor_position) {
         if mouse_over_inc || mouse_over_dec {
            let mut event_status = event::Status::Captured;
            match event {
               Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                  if mouse_over_dec {
                     self.state.decrease_pressed = true;
                     self.decrease_val(messages);
                  } else if mouse_over_inc {
                     self.state.increase_pressed = true;
                     self.increase_val(messages);
                  } else {
                     event_status = event::Status::Ignored;
                  }
               }
               Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                  if mouse_over_dec {
                     self.state.decrease_pressed = false;
                  } else if mouse_over_inc {
                     self.state.increase_pressed = false;
                  } else {
                     event_status = event::Status::Ignored;
                  }
               }
               _ => event_status = event::Status::Ignored
            }
            event_status
         } else {
            match event {
               Event::Keyboard(keyboard::Event::CharacterReceived(c)) 
                  if self.content.state().is_focused() && c.is_numeric() => {
                     let mut new_val = self.value.to_string();
                     match self.content.state().cursor().state(&Value::new(&new_val)) {
                        cursor::State::Index(idx) => new_val.insert(idx, c),
                        cursor::State::Selection{start, end} => new_val.replace_range(start..end, &c.to_string()),
                     }
                     
                     match T::from_str(&new_val) {
                        Ok(val) => {
                           if (self.bound.0..=self.bound.1).contains(&val) {
                              self.content.on_event(event.clone(), content, cursor_position, messages, renderer, clipboard)
                           } else {
                              event::Status::Ignored
                           }
                        },
                        Err(_) => event::Status::Ignored
                     }
                  },
               Event::Keyboard(keyboard::Event::KeyPressed {
                  key_code, ..
               }) if self.content.state().is_focused() => {
                  match key_code {
                     keyboard::KeyCode::Up => {
                        self.increase_val(messages);
                        // self.content.state().move_cursor_to_end();
                        event::Status::Captured
                     },
                     keyboard::KeyCode::Down => {
                        self.decrease_val(messages);
                        // self.content.state().move_cursor_to_end();
                        event::Status::Captured
                     },
                     _ => self.content.on_event(event.clone(), content, cursor_position, messages, renderer, clipboard)
                  }
               },
               _ => self.content.on_event(event.clone(), content, cursor_position, messages, renderer, clipboard)
            }
         }
      } else {
         event::Status::Ignored
      }
   }
}

#[derive(Default, Clone, Debug)]
pub struct State {
   input_state: text_input::State,
   mod_state: ModifierState,
}

impl State {
   pub fn new() -> Self {
      Self::default()
   }
}

#[derive(Default, Clone, Debug)]
pub struct ModifierState {
   decrease_pressed: bool,
   increase_pressed: bool,
}

pub trait Renderer: text_input::Renderer {
   type Style: Default;
   const DEFAULT_PADDING: u16;
   fn draw(&mut self, cursor_position: Point, state: &ModifierState, inc_bounds: Rectangle,
      dec_bounds: Rectangle, is_mouse_over: bool, is_decrease_disabled: bool, is_increase_disabled: bool, 
      content: Self::Output, style: &<Self as self::Renderer>::Style, font: Self::Font,
   ) -> Self::Output;
}

impl Renderer for iced_wgpu::Renderer {
   type Style = Box<dyn StyleSheet>;
   const DEFAULT_PADDING: u16 = 5;
   fn draw(&mut self, cursor_position: Point, state: &ModifierState, inc_bounds: Rectangle,
      dec_bounds: Rectangle, is_mouse_over: bool, is_decrease_disabled: bool, is_increase_disabled: bool,
      (content, _): Self::Output, style: &<Self as self::Renderer>::Style, font: Self::Font,
   ) -> Self::Output {
      let mouse_over_decrease = dec_bounds.contains(cursor_position);
      let mouse_over_increase = inc_bounds.contains(cursor_position);

      let decrease_btn_style = if is_decrease_disabled {
         style.disabled()
      } else if state.decrease_pressed {
         style.pressed()
      } else {
         style.active()
      };

      let increase_btn_style = if is_increase_disabled {
         style.disabled()
      } else if state.increase_pressed {
         style.pressed()
      } else {
         style.active()
      };

      // decrease button section
      let decrease_button_rect = Primitive::Quad {
         bounds: dec_bounds,
         background: decrease_btn_style
            .button_background
            .unwrap_or(Background::Color(Color::TRANSPARENT)),
         border_radius: 3.0,
         border_width: 0.,
         border_color: Color::TRANSPARENT,
      };
      let decrease_text = Primitive::Text {
         content: String::from("▼"),
         bounds: Rectangle {
            x: dec_bounds.center_x(),
            y: dec_bounds.center_y(),
            ..dec_bounds
         },
         font,
         size: dec_bounds.height * 0.9,
         color: decrease_btn_style.icon_color,
         horizontal_alignment: HorizontalAlignment::Center,
         vertical_alignment: VerticalAlignment::Center,
      };
      let decrease_btn = Primitive::Group {
         primitives: vec![decrease_button_rect, decrease_text],
      };

      // increase button section
      let increase_button_rect = Primitive::Quad {
         bounds: inc_bounds,
         background: increase_btn_style
            .button_background
            .unwrap_or(Background::Color(Color::TRANSPARENT)),
         border_radius: 3.0,
         border_width: 0.,
         border_color: Color::TRANSPARENT,
      };
      let increase_text = Primitive::Text {
         content: String::from("▲"),
         bounds: Rectangle {
            x: inc_bounds.center_x(),
            y: inc_bounds.center_y(),
            ..inc_bounds
         },
         font,
         size: inc_bounds.height * 0.9,
         color: increase_btn_style.icon_color,
         horizontal_alignment: HorizontalAlignment::Center,
         vertical_alignment: VerticalAlignment::Center,
      };
      let increase_btn = Primitive::Group {
         primitives: vec![increase_button_rect, increase_text],
      };

      (
         Primitive::Group {
            primitives: vec![content, decrease_btn, increase_btn],
         },
         if (mouse_over_decrease && !is_decrease_disabled) || (mouse_over_increase && !is_increase_disabled) {
            mouse::Interaction::Pointer
         } else if is_mouse_over {
            mouse::Interaction::Text
         } else {
            mouse::Interaction::default()
         },
      )
   }
}

impl<'a, T, Message, Renderer> From<NumberInput<'a, T, Message, Renderer>>
   for Element<'a, Message, Renderer>
where
   T: 'a + Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
   Message: 'a + Clone,
   Renderer: 'a + self::Renderer + container::Renderer + column::Renderer + row::Renderer,
{
   fn from(num_input: NumberInput<'a, T, Message, Renderer>) -> Self {
      Element::new(num_input)
   }
}
