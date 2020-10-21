use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
   layout, mouse, Background, Color, Element, Hasher, Layout, Length,
   Point, Size, Widget, Image
};

pub struct Card {
   header: String,
   body: Image,
   footer: String,
   max_width: usize,
   max_height: usize,
   min_width: usize,
   min_height: usize,
}

impl Card {
   pub fn new(body: Image) -> Self {
      Self {
         header: String::new(),
         body,
         footer: String::new(),
         max_width: 0,
         max_height: 0,
         min_width: 0,
         min_height: 0,
      }
   }

   pub fn body(mut self, body: Image) -> Self {
      self.body = body;
      self
   }

   pub fn header(mut self, header: String) -> Self {
      self.header = header;
      self
   }

   pub fn footer(mut self, footer: String) -> Self {
      self.footer = footer;
      self
   }

   pub fn max_width(mut self, max_width: usize) -> Self {
      self.max_width = max_width;
      self
   }

   pub fn max_height(mut self, max_height: usize) -> Self {
      self.max_height = max_height;
      self
   }

   pub fn min_width(mut self, min_width: usize) -> Self {
      self.min_width = min_width;
      self
   }

   pub fn min_height(mut self, min_height: usize) -> Self {
      self.min_height = min_height;
      self
   }
}

impl<Message, B: Backend> Widget<Message, B> for Card {
   fn width(&self) -> Length {
      Length::Shrink
   }

   fn height(&self) -> Length {
      Length::Shrink
   }

   fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
      layout::Node::new(Size::new(self.max_width as f32, self.max_height as f32))
   }

   fn hash_layout(&self, state: &mut Hasher) {
      use std::hash::Hash;
      self.max_width.hash(state);
      self.max_height.hash(state);
   }

   fn draw(&self, renderer: &mut Renderer<B>, defaults: &Defaults, layout: Layout<'_>, cursor_position: Point) -> (Primitive, mouse::Interaction) {
      (
         Primitive::Quad {
            bounds: layout.bounds(),
            background: Background::Color(Color::from_rgba8(0, 145, 234, 0.4)),
            border_radius: 10,
            border_width: 0,
            border_color: Color::TRANSPARENT
         },
         mouse::Interaction::Pointer
      )
   }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for Card 
where B: Backend
{
   fn into(self) -> Element<'a, Message, Renderer<B>> {
      Element::new(self)
   }
}