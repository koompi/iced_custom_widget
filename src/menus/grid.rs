use iced_native::{
   layout::{Limits, Node},
   Align, Clipboard, Element, Event, Hasher, Layout, Length, Point, Size, Widget
};
use std::{any::TypeId, hash::Hash, iter};
use iced_graphics::{Backend, Defaults, Renderer};
pub struct Grid<'a, Message, Renderer> {
   cols: usize,
   spacing: u16,
   padding: u16,
   margin: u16,
   width: Length,
   height: Length,
   max_width: u32,
   max_height: u32,
   align_items: Align,
   children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer> {
   pub fn new() -> Self {
      Self::with_children(Vec::new())
   }

   pub fn with_children(children: Vec<Element<'a, Message, Renderer>>) -> Self {
      Grid {
         cols: 2,
         spacing: 0,
         padding: 0,
         margin: 0,
         width: Length::Shrink,
         height: Length::Shrink,
         max_width: u32::MAX,
         max_height: u32::MAX,
         align_items: Align::Start,
         children,
      }
   }

   pub fn cols(mut self, units: usize) -> Self {
      self.cols = units;
      self
   }

   pub fn spacing(mut self, units: u16) -> Self {
      self.spacing = units;
      self
   }

   pub fn padding(mut self, units: u16) -> Self {
      self.padding = units;
      self
   }

   pub fn margin(mut self, units: u16) -> Self {
      self.margin = units;
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

   pub fn max_width(mut self, max_width: u32) -> Self {
      self.max_width = max_width;
      self
   }

   pub fn max_height(mut self, max_height: u32) -> Self {
      self.max_height = max_height;
      self
   }

   pub fn align_items(mut self, align: Align) -> Self {
      self.align_items = align;
      self
   }

   pub fn push<E>(mut self, child: E) -> Self
   where
      E: Into<Element<'a, Message, Renderer>>,
   {
      self.children.push(child.into());
      self
   }
}

impl<'a, Message, B> Widget<Message, B> for Grid<'a, Message, B>
where
   B: Backend,
{
   fn width(&self) -> Length {
      self.width
   }

   fn height(&self) -> Length {
      self.height
   }

   fn layout(&self, renderer: &Renderer<B>, limits: &Limits) -> Node {
      if self.children.is_empty() {
         return Node::new(Size::ZERO);
      }
      let limits = limits
         .max_width(self.max_width)
         .max_height(self.max_height)
         .width(self.width)
         .height(self.height);

      // layout::flex::resolve(axis: Axis, renderer: &Renderer, limits: &Limits, padding: f32, spacing: f32, align_items: Align, items: &[Element<'_, Message, Renderer>])

      let mut layouts = Vec::with_capacity(self.children.len());
      let mut cols_width = Vec::<f32>::with_capacity(self.cols);

      self.children.iter().enumerate().for_each(|(i, element)| {
         let layout = element.layout(renderer, limits).size();
         layouts.push(layout);

         if let Some(col_width) = cols_width.get_mut(i) {
            *col_width = col_width.max(layout.width);
         } else {
            cols_width.insert(i, layout.width);
         }
      });

      let cols_align = iter::once(&0.)
         .chain(cols_width.iter())
         .scan(0., |state, width| {
            *state += width;
            Some(*state)
         });

      let grid_width = cols_width.iter().sum();
      build_grid(self.cols, cols_align, layouts.into_iter(), grid_width)
   }
   fn on_event(
      &mut self,
      event: Event,
      layout: Layout<'_>,
      cursor_position: Point,
      messages: &mut Vec<Message>,
      renderer: &Renderer<B>,
      clipboard: Option<&dyn Clipboard>,
   ) {
      self
         .children
         .iter_mut()
         .zip(layout.children())
         .for_each(|(child, layout)| {
            child.on_event(
               event.clone(),
               layout,
               cursor_position,
               messages,
               renderer,
               clipboard,
            )
         });
   }

   fn draw(
      &self,
      renderer: &mut Renderer<B>,
      defaults: &Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
   ) -> Renderer<B>::Output {
      renderer.draw(defaults, layout, cursor_position, &self.children)
   }

   fn hash_layout(&self, state: &mut Hasher) {
      TypeId::of::<Grid<'_, (), ()>>().hash(state);

      for element in &self.children {
         element.hash_layout(state);
      }
   }
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for Grid<'a, Message, B>
where
   B: Backend,
{
   fn into(self) -> Element<'a, Message, Renderer<B>> {
      Element::new(self)
   }
}

fn build_grid(
   cols: usize,
   cols_align: impl Iterator<Item = f32> + Clone,
   layouts: impl Iterator<Item = Size> + ExactSizeIterator,
   grid_width: f32,
) -> Node {
   let mut nodes = Vec::with_capacity(layouts.len());
   let mut grid_height = 0.;
   let mut row_height = 0.;

   for ((col, col_align), size) in (0..cols).zip(cols_align).cycle().zip(layouts) {
      if col == 0 {
         grid_height += row_height;
         row_height = 0.;
      }

      let mut node = Node::new(size);
      node.move_to(Point::new(col_align, grid_height));
      nodes.push(node);
      row_height = row_height.max(size.height);
   }

   grid_height += row_height;
   Node::with_children(Size::new(grid_width, grid_height), nodes)
}