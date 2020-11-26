use super::{
   error::Result,
   table_column::{TableColumn, TableOptions, TableOrder},
};
use crate::styles::table::StyleSheet;
use iced_graphics::{backend, Backend, Primitive};
use iced_native::{
   event::{self, Event}, layout::{Limits, Node}, mouse, text, row, column,
   Color, Element, Hasher, HorizontalAlignment, Layout, Length, Point, Rectangle, Size, VerticalAlignment,
   Widget,
};
use serde::Serialize;
use serde_value::Value;

pub trait TableData: 'static + Default + Clone + Ord + Serialize {
   fn get_field_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Clone, PartialEq, Default)]
pub struct TableState {
   pub orders: Vec<TableOrder>,
}

pub struct Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   state: TableState,
   columns: Vec<TableColumn>,
   data: &'a mut Vec<T>,
   selected_row: Option<T>,
   padding: u16,
   // column_max_width: f32,
   option: Option<TableOptions>,
   text_size: Option<u16>,
   font: Renderer::Font,
   style: Renderer::Style,
}

impl<'a, T, Renderer> Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   pub fn new(columns: Vec<TableColumn>, data: &'a mut Vec<T>) -> Self {
      let cols_num = columns.len();
      Self {
         state: TableState {
            orders: vec![TableOrder::default(); cols_num],
         },
         columns,
         data,
         selected_row: None,
         padding: Renderer::DEFAULT_PADDING,
         option: None,
         text_size: None,
         font: Renderer::Font::default(),
         style: Renderer::Style::default(),
      }
   }

   pub fn option(mut self, option: TableOptions) -> Self {
      self.option = Some(option);
      self
   }

   pub fn text_size(mut self, text_size: u16) -> Self {
      self.text_size = Some(text_size);
      self
   }

   pub fn padding(mut self, padding: u16) -> Self {
      self.padding = padding;
      self
   }

   pub fn font(mut self, font: Renderer::Font) -> Self {
      self.font = font;
      self
   }

   pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
      self.style = style.into();
      self
   }

   fn is_orderable(&self) -> bool {
      if let Some(option) = &self.option {
         option.orderable
      } else {
         false
      }
   }

   fn trigger_sort_column(&mut self, idx: usize) {
      use TableOrder::*;

      for (i, x) in self.state.orders.iter_mut().enumerate() {
         if i != idx {
            *x = Unordered
         } else {
            *x = x.toggle()
         }
      }
      let name = &self.columns[idx].name;
      match self.state.orders[idx] {
         Unordered => self.data.sort(),
         Ascending => self.data.sort_by_cached_key(|x| x.get_field_value(name.as_ref()).unwrap()),
         Descending => self
            .data
            .sort_by_cached_key(|x| std::cmp::Reverse(x.get_field_value(name.as_ref()).unwrap())),
      }
   }
}

impl<'a, Message, Renderer, T> Widget<Message, Renderer> for Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   fn width(&self) -> Length {
      Length::Shrink
   }

   fn height(&self) -> Length {
      Length::Shrink
   }

   fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
      let padding = f32::from(self.padding);
      let limits = limits.width(Length::Shrink).height(Length::Shrink);
      let bounds = limits.resolve(Size::INFINITY);
      let text_size = self.text_size.unwrap_or(renderer.default_size());

      let mut header_nodes = Vec::new();
      let mut header_size = Size::ZERO;
      for column in &self.columns {
         let (width, height) = renderer.measure(&column.to_string(), text_size, self.font, bounds);
         let size = {
            let intrinsic = Size::new(width+f32::from(text_size)+padding, height+padding);
            limits.resolve(intrinsic)
         };
         let mut node = Node::new(size);
         node.move_to(Point::new(header_size.width, 0.0));
         header_nodes.push(node);
         header_size.width += size.width;
         header_size.height = header_size.height.max(size.height);
      }
      let mut header = Node::with_children(header_size, header_nodes);
      header.move_to(Point::new(1.0, 1.0));

      let mut body_nodes = Vec::new();
      let mut table_size = Size::new(0.0, header_size.height);
      self.data.iter().for_each(|record| {
         let mut record_nodes = Vec::new();
         let mut record_size = Size::ZERO;
         self
            .columns
            .iter()
            .map(|c| c.name.as_str())
            .map(|name| record.get_field_value(name))
            .filter_map(|h| h.ok())
            .for_each(|value| {
               let (width, height) = renderer.measure("&value", text_size, self.font, bounds);
               let size = {
                  let intrinsic = Size::new(width+f32::from(text_size)+padding, height+padding);
                  limits.resolve(intrinsic)
               };
               let mut cell = Node::new(size);
               cell.move_to(Point::new(record_size.width, table_size.height));
               record_nodes.push(cell);
               record_size.width += size.width;
               record_size.height = record_size.height.max(size.height);
            });
         let record = Node::with_children(record_size, record_nodes);
         body_nodes.push(record);
         table_size.width = table_size.width.max(record_size.width);
         table_size.height += record_size.height;
      });
      let body = Node::with_children(table_size, body_nodes);

      table_size.width = header_size.width.max(table_size.width)+2.0;
      Node::with_children(table_size, vec![header, body])
   }

   fn draw(
      &self,
      renderer: &mut Renderer,
      defaults: &Renderer::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
   ) -> Renderer::Output {
      self::Renderer::draw(
         renderer,
         defaults,
         layout,
         cursor_position,
         viewport,
         &self.columns,
         &self.data,
         self.text_size.unwrap_or(renderer.default_size()),
         self.padding,
         self.font,
         &self.style,
      )
   }

   fn hash_layout(&self, state: &mut Hasher) {
      use std::hash::Hash;
      struct Marker;
      std::any::TypeId::of::<Marker>().hash(state);
      self
         .columns
         .iter()
         .map(ToString::to_string)
         // .zip(self.data.clone())
         .for_each(|column| {
            column.hash(state);
            // record.get_field_value(&column).unwrap().hash(state);
         });
   }
}

pub trait Renderer: text::Renderer {
   type Style: Default;

   const DEFAULT_PADDING: u16;

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
      data: &[T],
      text_size: u16,
      padding: u16,
      font: Self::Font,
      style: &Self::Style,
   ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
   B: Backend + backend::Text,
{
   type Style = Box<dyn StyleSheet>;

   const DEFAULT_PADDING: u16 = 4;

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
      data: &[T],
      text_size: u16,
      padding: u16,
      font: Self::Font,
      style: &Self::Style,
   ) -> Self::Output {
      let header_layout = layout.children().next().unwrap();
      let body_layout = layout.children().next().unwrap();
      let header_bounds = header_layout.bounds();
      let header_mouse_over = header_bounds.contains(cursor_position);

      let styling = if header_mouse_over {
         style.header_hoverd()
      } else {
         style.active()
      };

      let background = Primitive::Quad {
         bounds: layout.bounds(),
         background: styling.background,
         border_color: styling.border_color,
         border_width: styling.border_width,
         border_radius: styling.border_radius,
      };
      let header_background = Primitive::Quad {
         bounds: header_bounds,
         background: styling.header_background,
         border_color: Color::TRANSPARENT,
         border_width: 0.0,
         border_radius: styling.border_radius,
      };

      let header_section = Primitive::Group {
         primitives: columns
            .iter()
            .map(ToString::to_string)
            .zip(header_layout.children())
            .map(|(content, layout)| {
               let bounds = layout.bounds();
               Primitive::Text {
                  content,
                  size: f32::from(text_size),
                  font,
                  color: styling.text_color,
                  bounds: Rectangle {
                     x: bounds.x + f32::from(padding),
                     y: bounds.center_y(),
                     ..bounds
                  },
                  horizontal_alignment: HorizontalAlignment::Left,
                  vertical_alignment: VerticalAlignment::Center,
               }
            })
            .collect(),
      };

      let mut body_records = Vec::with_capacity(data.len());
      for (record, record_layout) in data.iter().zip(body_layout.children()) {
         let record_cells = columns
            .iter()
            .map(|c| c.name.as_str())
            .map(|name| record.get_field_value(name))
            .filter_map(|h| h.ok())
            .zip(record_layout.children())
            .fold(Vec::with_capacity(columns.len()), |mut record_cells, (value, cell_layout)| {
               let bounds = cell_layout.bounds();
               record_cells.push(
                  Primitive::Text {
                     content: "value.to_string()".to_string(),
                     size: f32::from(text_size),
                     font,
                     color: styling.text_color,
                     bounds: Rectangle {
                        x: bounds.x + f32::from(padding),
                        y: bounds.center_y(),
                        ..bounds
                     },
                     horizontal_alignment: HorizontalAlignment::Left,
                     vertical_alignment: VerticalAlignment::Center,
                  }
               );
               println!("draw a cell");
               record_cells
            });
         body_records.push(Primitive::Group{ primitives: record_cells });
      }
      let body_section = Primitive::Group{ primitives: body_records };
      println!("{:#?}", body_section);
      (
         Primitive::Group{ primitives: vec![background, header_background, body_section, header_section] },
         if header_mouse_over {
            mouse::Interaction::Pointer
         } else {
            mouse::Interaction::default()
         },
      )
   }
}

#[derive(Debug)]
pub enum TableEvent {
   SortColumn(usize),
}

impl<'a, T, Message, Renderer> From<Table<'a, T, Renderer>> for Element<'a, Message, Renderer>
where
   T: TableData,
   Renderer: 'a + self::Renderer,
   Message: Clone + 'static,
{
   fn from(table: Table<'a, T, Renderer>) -> Element<'a, Message, Renderer> {
      Element::new(table)
   }
}
