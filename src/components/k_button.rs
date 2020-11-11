use iced_native::{
   layout::{
      Node, Limits
   },
   Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget, mouse, Background, Color, Vector
};
use std::hash::Hash;
use crate::styles::k_button::StyleSheet;
use iced_wgpu::Primitive;

pub struct KButtton<'a, Message, Renderer: self::Renderer> {
   state: &'a mut State,
   content: Element<'a, Message, Renderer>,
   on_press: Option<Message>,
   width: Length,
   height: Length,
   min_width: u32,
   min_height: u32,
   padding: u16,
   style: Renderer::Style
}

impl<'a, Message, Renderer> KButtton<'a, Message, Renderer>
where 
   Message: Clone,
   Renderer: self::Renderer 
{
   pub fn new(state: &'a mut State, content: impl Into<Element<'a, Message, Renderer>>) -> Self {
      Self {
         state,
         content: content.into(),
         on_press: None,
         width: Length::Shrink,
         height: Length::Shrink,
         min_width: 0,
         min_height: 0,
         padding: Renderer::DEFAULT_PADDING,
         style: Renderer::Style::default()
      }
   }

   pub fn on_press(mut self, on_press: Message) -> Self {
      self.on_press = Some(on_press);
      self
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

   pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
      self.style = style.into();
      self
   }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
   for KButtton<'a, Message, Renderer>
where
   Message: Clone,
   Renderer: self::Renderer 
{
   fn width(&self) -> Length {
      self.width
   }

   fn height(&self) -> Length {
      self.height
   }

   fn layout(
      &self,
      renderer: &Renderer,
      limits: &Limits,
   ) -> Node {
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
         layout.children().next()
      )
   }

   fn hash_layout(&self, state: &mut Hasher) {
      self.width.hash(state);
      self.height.hash(state);
      self.content.hash_layout(state);
   }

   fn on_event(
      &mut self, 
      event: Event, 
      layout: Layout<'_>, 
      cursor_position: Point, 
      messages: &mut Vec<Message>, 
      _renderer: &Renderer, 
      _clipboard: Option<&dyn Clipboard>
   ) {
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
               let is_pressed = self.state.is_pressed && bounds.contains(cursor_position);
               self.state.is_pressed = false;

               if is_pressed {
                  messages.push(on_press);
               }
            }
         }
         _ => {}
      }
   }
}

#[derive(Default, Debug, Clone)]
pub struct State {
   is_pressed: bool
}

impl State {
   pub fn new() -> Self {
      Self::default()
   }
}

pub trait Renderer : iced_native::Renderer {
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
      content_layout: Option<Layout<'_>>
   ) -> Self::Output;
}

impl Renderer for iced_wgpu::Renderer {
   const DEFAULT_PADDING: u16 = 8;

   type Style = Box<dyn StyleSheet>;

   fn draw<Message>(
      &mut self,
      defaults: &Self::Defaults,
      bounds: Rectangle,
      cursor_position: Point,
      is_disabled: bool,
      is_pressed: bool,
      style: &Self::Style,
      content: &Element<'_, Message, Self>,
      content_layout: Option<Layout<'_>>
   ) -> Self::Output {
      let is_hovered = bounds.contains(cursor_position);
      let styling = if is_disabled {
         style.disabled()
      } else if is_hovered {
         if is_pressed {
            style.pressed()
         } else {
            style.hovered()
         }
      } else {
         style.active()
      };

      let (content, _) = match content_layout {
         Some(layout) => content.draw(self, defaults, layout, cursor_position, &bounds),
         None => (
            Primitive::None,
            mouse::Interaction::default()
         )
      };
      (
         if styling.background.is_some() || styling.border_width > 0 {
            let background = Primitive::Quad {
               bounds,
               background: styling.background.unwrap_or(Background::Color(Color::TRANSPARENT)),
               border_radius: styling.border_radius,
               border_width: styling.border_width,
               border_color: styling.border_color
            };

            if styling.shadow_offset == Vector::default() {
               Primitive::Group { primitives: vec![background,content]}
            } else {
               let shadow = Primitive::Quad {
                  bounds: Rectangle {
                     x: bounds.x + styling.shadow_offset.x,
                     y: bounds.y + styling.shadow_offset.y,
                     ..bounds
                  },
                  background: Background::Color([0.5, 0.5, 0.5].into()),
                  border_radius: styling.border_radius,
                  border_width: 0,
                  border_color: Color::TRANSPARENT
               };

               Primitive::Group{ primitives: vec![shadow, background, content] }
            }
         } else {
            content
         },
         if is_hovered && !is_disabled {
            mouse::Interaction::Pointer
         } else {
            mouse::Interaction::default()
         }
      )
   }
}

impl<'a, Message, Renderer> From<KButtton<'a, Message, Renderer>> 
   for Element<'a, Message, Renderer> 
where 
   Message: 'a + Clone,
   Renderer: 'a + self::Renderer
{
   fn from(k_button: KButtton<'a, Message, Renderer>) -> Self {
      Element::new(k_button)
   }
}