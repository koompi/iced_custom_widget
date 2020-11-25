use crate::styles::outline_button::StyleSheet;
use iced_graphics::{defaults, Backend, Defaults, Primitive};
use iced_native::{
   event::{self, Event},
   layout::{Limits, Node},
   mouse, Background, Clipboard, Color, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct OutlineButton<'a, Message, Renderer>
where
   Renderer: self::Renderer,
{
   state: &'a mut State,
   content: Element<'a, Message, Renderer>,
   on_press: Option<Message>,
   width: Length,
   height: Length,
   min_width: u32,
   min_height: u32,
   padding: u16,
   style: Renderer::Style,
}

impl<'a, Message, Renderer> OutlineButton<'a, Message, Renderer>
where
   Message: Clone,
   Renderer: self::Renderer,
{
   pub fn new<E>(state: &'a mut State, content: E) -> Self
   where
      E: Into<Element<'a, Message, Renderer>>,
   {
      Self {
         state,
         content: content.into(),
         on_press: None,
         width: Length::Shrink,
         height: Length::Shrink,
         min_width: 0,
         min_height: 0,
         padding: Renderer::DEFAULT_PADDING,
         style: Renderer::Style::default(),
      }
   }

   pub fn width(mut self, width: Length) -> Self {
      self.width = width;
      self
   }

   pub fn height(mut self, height: Length) -> Self {
      self.height = height;
      self
   }

   pub fn min_width(mut self, min_width: u32) -> Self {
      self.min_width = min_width;
      self
   }

   pub fn min_height(mut self, min_height: u32) -> Self {
      self.min_height = min_height;
      self
   }

   pub fn padding(mut self, padding: u16) -> Self {
      self.padding = padding;
      self
   }

   pub fn on_press(mut self, msg: Message) -> Self {
      self.on_press = Some(msg);
      self
   }

   pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
      self.style = style.into();
      self
   }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for OutlineButton<'a, Message, Renderer>
where
   Message: Clone,
   Renderer: self::Renderer,
{
   fn width(&self) -> Length {
      self.width
   }

   fn height(&self) -> Length {
      self.height
   }

   fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
      // let button = Button::<Message, Renderer>::new(&mut self.state, self.content)
      //     .width(self.width)
      //     .height(self.height)
      //     .min_width(self.min_width)
      //     .min_height(self.min_height)
      //     .padding(self.padding);
      // if let Some(on_press) = self.on_press {
      //     button.on_press(on_press);
      // }
      // button.layout(renderer, limits)
      let padding = f32::from(self.padding);
      let limits = limits
         .min_width(self.min_width)
         .min_height(self.min_height)
         .width(self.width)
         .height(self.height)
         .pad(padding);

      let mut content = self.content.layout(renderer, &limits);
      content.move_to(Point::new(padding, padding));

      let size = limits.resolve(content.size()).pad(padding);

      Node::with_children(size, vec![content])
   }

   fn on_event(
      &mut self,
      event: Event,
      layout: Layout<'_>,
      cursor_position: Point,
      messages: &mut Vec<Message>,
      _renderer: &Renderer,
      _clipboard: Option<&dyn Clipboard>,
   ) -> event::Status {
      match event {
         Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            if self.on_press.is_some() {
               let bounds = layout.bounds();

               self.state.is_pressed = bounds.contains(cursor_position);
            }
         }
         Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            if let Some(on_press) = self.on_press.clone() {
               let bounds = layout.bounds();

               let is_clicked = self.state.is_pressed && bounds.contains(cursor_position);

               self.state.is_pressed = false;

               if is_clicked {
                  messages.push(on_press);
                  return event::Status::Captured;
               }
            }
         }
         _ => {}
      }

      event::Status::Ignored
   }

   fn draw(
      &self,
      renderer: &mut Renderer,
      defaults: &Renderer::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      _viewport: &Rectangle,
   ) -> Renderer::Output {
      renderer.draw(
         defaults,
         layout.bounds(),
         cursor_position,
         self.on_press.is_none(),
         self.state.is_pressed,
         &self.style,
         &self.content,
         layout.children().next().unwrap(),
      )
   }

   fn hash_layout(&self, state: &mut Hasher) {
      use std::hash::Hash;
      struct Marker;
      std::any::TypeId::of::<Marker>().hash(state);

      self.width.hash(state);
      self.content.hash_layout(state);
   }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
   is_pressed: bool,
}

impl State {
   pub fn new() -> State {
      State::default()
   }
}

pub trait Renderer: iced_native::Renderer {
   const DEFAULT_PADDING: u16;
   type Style: Default;
   fn draw<Message>(
      &mut self,
      defaults: &Self::Defaults,
      bounds: Rectangle,
      cursor_position: Point,
      is_disabled: bool,
      is_pressed: bool,
      style: &Self::Style,
      content: &Element<'_, Message, Self>,
      content_layout: Layout<'_>,
   ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
   B: Backend,
{
   const DEFAULT_PADDING: u16 = 8;

   type Style = Box<dyn StyleSheet>;

   fn draw<Message>(
      &mut self,
      _defaults: &Self::Defaults,
      bounds: Rectangle,
      cursor_position: Point,
      is_disabled: bool,
      is_pressed: bool,
      style: &Self::Style,
      content: &Element<'_, Message, Self>,
      content_layout: Layout<'_>,
   ) -> Self::Output {
      let is_mouse_over = bounds.contains(cursor_position);

      let styling = if is_disabled {
         style.disabled()
      } else if is_mouse_over {
         if is_pressed {
            style.pressed()
         } else {
            style.hovered()
         }
      } else {
         style.active()
      };

      let (content, _) = content.draw(
         self,
         &Defaults {
            text: defaults::Text {
               color: styling.text_color,
            },
         },
         content_layout,
         cursor_position,
         &bounds,
      );

      (
         if styling.border_width > 0.0 {
            let background = Primitive::Quad {
               bounds,
               background: if is_disabled {
                  Background::Color(Color::from_rgba8(240, 243, 244, 0.3))
               } else if is_pressed {
                  Background::Color(styling.border_color)
               } else {
                  Background::Color(Color::from_rgba8(255, 255, 255, 0.3))
               },
               border_radius: styling.border_radius,
               border_width: styling.border_width,
               border_color: styling.border_color,
            };

            Primitive::Group {
               primitives: vec![background, content],
            }
         } else {
            content
         },
         if is_mouse_over && !is_disabled {
            mouse::Interaction::Pointer
         } else {
            mouse::Interaction::default()
         },
      )
   }
}

impl<'a, Message, Renderer> From<OutlineButton<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
   Message: 'a + Clone,
   Renderer: 'a + self::Renderer,
{
   fn from(outline_button: OutlineButton<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
      Element::new(outline_button)
   }
}
