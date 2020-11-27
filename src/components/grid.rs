use iced_graphics::Primitive;
use iced_native::{
   container, event::{self, Event}, layout::{flex::{self, Axis}, Limits, Node}, mouse, overlay, 
   Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};

/// A container that produces a grid layout.
/// A scrollable, 2D array of widgets.
pub struct Grid<'a, Message, Renderer> {
   width: Length,
   height: Length,
   spacing: u16,
   padding: u16,
   columns: Option<usize>,
   column_width: Option<u16>,
   children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
   Renderer: self::Renderer,
{
   pub fn new() -> Self {
      Self::with_children(Vec::new())
   }

   pub fn with_children(children: Vec<Element<'a, Message, Renderer>>) -> Self {
      Self {
         width: Length::Fill,
         height: Length::Shrink,
         spacing: Renderer::DEFAULT_SPACING,
         padding: Renderer::DEFAULT_PADDING,
         columns: None,
         column_width: None,
         children,
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

   pub fn padding(mut self, padding: u16) -> Self {
      self.padding = padding;
      self
   }

   pub fn spacing(mut self, spacing: u16) -> Self {
      self.spacing = spacing;
      self
   }

   pub fn columns(mut self, columns: usize) -> Self {
      if columns == 0 {
         self.columns = None;
      } else {
         self.columns = Some(columns);
      }
      self
   }

   pub fn column_width(mut self, column_width: u16) -> Self {
      self.column_width = Some(column_width);
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

impl<'a, Message, Renderer> Widget<Message, Renderer> for Grid<'a, Message, Renderer>
where
   Renderer: self::Renderer,
{
   fn width(&self) -> Length {
      self.width
   }

   fn height(&self) -> Length {
      self.height
   }

   fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
      if self.children.is_empty() {
         Node::new(Size::ZERO)
      } else {
         let padding = f32::from(self.padding);
         let spacing = f32::from(self.spacing);
         let limits = limits.width(self.width).height(self.height);
         if let Some(columns) = self.columns {
            let mut column_widths = Vec::<f32>::with_capacity(columns);

            for (column, element) in (0..columns).cycle().zip(&self.children) {
               if let Some(column_width) = column_widths.get_mut(column) {
                  *column_width = column_width.max(element.layout(renderer, &limits).size().width);
               } else {
                  column_widths.insert(column, element.layout(renderer, &limits).size().width);
               }
            }

            let column_aligns = std::iter::once(&0.)
               .chain(column_widths.iter().take(column_widths.len() - 1))
               .scan(0., |state, width| {
                  *state += width;
                  Some(*state)
               });

            let mut nodes = Vec::with_capacity(self.children.len());
            let mut grid_height = 0.;
            let mut row_height = 0.;
            let mut offset = 0.;

            for ((column, column_align), element) in column_aligns.enumerate().cycle().zip(&self.children) {
               let mut node = element.layout(renderer, &limits);
               let size = node.size();
               if column == 0 {
                  grid_height += row_height;
                  row_height = 0.;
                  offset = 0.;
                  node.move_to(Point::new(column_align + padding, grid_height + padding));
               } else {
                  offset += spacing;
                  node.move_to(Point::new(column_align + padding + offset, grid_height + padding));
               }
               row_height = row_height.max(size.height + spacing);
               nodes.push(node);
            }

            grid_height += row_height;
            let grid_width: f32 = column_widths.into_iter().sum();

            Node::with_children(
               Size::new(
                  grid_width + (padding * 2.) + (spacing * (columns as f32 - 1.)),
                  grid_height + padding,
               ),
               nodes,
            )
         } else if let Some(column_width) = self.column_width {
            let column_width = f32::from(column_width);
            let max_width = limits.max().width;
            let columns = (max_width / column_width).floor() as u16;
            let mut nodes = Vec::with_capacity(self.children.len());
            let mut grid_height = 0.;
            let mut row_height = 0.;
            let mut offset = 0.;

            for ((idx, column), element) in (0..columns).cycle().enumerate().zip(&self.children) {
               let mut node = element.layout(renderer, &limits);
               let size = node.size();
               offset = (column_width - size.width) / 2.;
               if column == 0 {
                  grid_height += row_height;
                  row_height = 0.;
               }
               if padding != 0. {
                  offset = padding;
               }
               if idx == column as usize {
                  node.move_to(Point::new((column as f32) * column_width + offset, grid_height + padding));
                  row_height = row_height.max(size.height + padding);
               } else {
                  node.move_to(Point::new((column as f32) * column_width + offset, grid_height + spacing));
                  row_height = row_height.max(size.height + spacing);
               }
               nodes.push(node);
            }
            grid_height += row_height;
            let grid_width = (columns as f32) * column_width;

            if padding != 0. {
               Node::with_children(Size::new(grid_width + padding, grid_height + padding), nodes)
            } else {
               Node::with_children(Size::new(grid_width, grid_height + padding), nodes)
            }
         } else {
            flex::resolve(Axis::Horizontal, renderer, &limits, padding, 8., Align::Start, &self.children)
         }
      }
   }

   fn draw(
      &self,
      renderer: &mut Renderer,
      defaults: &Renderer::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
   ) -> Renderer::Output {
      self::Renderer::draw(renderer, defaults, layout, cursor_position, viewport, &self.children)
   }

   fn hash_layout(&self, state: &mut Hasher) {
      use std::{any::TypeId, hash::Hash};

      struct Marker;
      TypeId::of::<Marker>().hash(state);

      self.padding.hash(state);
      self.children.iter().for_each(|child| {
         child.hash_layout(state);
      });
   }

   fn on_event(
      &mut self,
      event: Event,
      layout: Layout<'_>,
      cursor_position: Point,
      messages: &mut Vec<Message>,
      renderer: &Renderer,
      clipboard: Option<&dyn Clipboard>,
   ) -> event::Status {
      let event_status = event::Status::Ignored;
      self
         .children
         .iter_mut()
         .zip(layout.children())
         .map(|(child, layout)| child.on_event(event.clone(), layout, cursor_position, messages, renderer, clipboard))
         .fold(event_status, event::Status::merge)
   }

   fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
      self
         .children
         .iter_mut()
         .zip(layout.children())
         .filter_map(|(child, layout)| child.overlay(layout))
         .next()
   }
}

pub trait Renderer: iced_native::Renderer + container::Renderer + Sized {
   const DEFAULT_PADDING: u16;
   const DEFAULT_SPACING: u16;

   fn draw<Message>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      children: &[Element<'_, Message, Self>],
   ) -> Self::Output;
}

impl Renderer for iced_wgpu::Renderer
{
   const DEFAULT_PADDING: u16 = 0;
   const DEFAULT_SPACING: u16 = 8;

   fn draw<Message>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      children: &[Element<'_, Message, Self>],
   ) -> Self::Output {
      let mut mouse_interaction = mouse::Interaction::default();

      (
         Primitive::Group {
            primitives: children
               .iter()
               .zip(layout.children())
               .map(|(child, layout)| {
                  let (primitive, new_mouse_interaction) = child.draw(self, defaults, layout, cursor_position, viewport);

                  if new_mouse_interaction > mouse_interaction {
                     mouse_interaction = new_mouse_interaction;
                  }

                  primitive
               })
               .collect(),
         },
         mouse_interaction,
      )
   }
}

impl<'a, Message, Renderer> From<Grid<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
   Renderer: 'a + self::Renderer,
   Message: 'a,
{
   fn from(grid: Grid<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
      Element::new(grid)
   }
}
