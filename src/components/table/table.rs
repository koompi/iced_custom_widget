use super::{
   error::Result,
   table_column::{TableColumn, TableOptions, TableOrder},
};
use crate::styles::table::StyleSheet;
use iced::Vector;
use iced_graphics::Primitive;
use iced_native::{
   event::{self, Event}, layout::{Limits, Node}, mouse, text, Color, Element, Hasher, Clipboard,
   HorizontalAlignment, Layout, Length, Point, Rectangle, Size, VerticalAlignment, Widget, Background,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub trait TableData: 'static + Default + Clone + Ord + Serialize + DeserializeOwned {
   fn get_field_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Clone, PartialEq)]
pub struct State {
   pub orders: Vec<TableOrder>,
}

pub struct Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   state: State,
   columns: Vec<TableColumn>,
   data: &'a mut Vec<T>,
   selected_row: Option<usize>,
   padding: u16,
   header_spacing: u16,
   column_max_width: u32,
   option: TableOptions,
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
      Self {
         state: State {
            orders: vec![TableOrder::default(); columns.len()]
         },
         columns,
         data,
         selected_row: None,
         padding: Renderer::DEFAULT_PADDING,
         header_spacing: Renderer::DEFAULT_HEADER_SPACING,
         column_max_width: Renderer::DEFAULT_COL_MAX_WIDTH,
         option: TableOptions::default(),
         text_size: None,
         font: Renderer::Font::default(),
         style: Renderer::Style::default(),
      }
   }

   pub fn option(mut self, option: TableOptions) -> Self {
      self.option = option;
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

   pub fn header_spacing(mut self, spacing: u16) -> Self {
      self.header_spacing = spacing;
      self
   }

   pub fn column_max_width(mut self, column_max_width: u32) -> Self {
      self.column_max_width = column_max_width;
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
      self.option.orderable
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
         Ascending => self.data.sort_by_cached_key(|x| x.get_field_value(&name).unwrap().to_string()),
         Descending => self.data.sort_by_cached_key(|x| std::cmp::Reverse(x.get_field_value(&name).unwrap().to_string())),
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
      for (idx, column) in self.columns.iter().enumerate() {
         let label = column.to_string();
         let formatted_label = match self.state.orders[idx] {
            TableOrder::Unordered => label,
            TableOrder::Ascending => format!("{} ▲", label),
            TableOrder::Descending => format!("{} ▼", label),
         };
         let (width, height) = renderer.measure(&formatted_label, text_size, self.font, bounds);
         let size = {
            let intrinsic = Size::new(width+f32::from(text_size), height);
            limits.resolve(intrinsic).pad(padding)
         };
         let mut node = Node::new(size);
         node.move_to(Point::new(header_size.width, 0.0));
         header_size.width += size.width;
         header_size.height = header_size.height.max(size.height);
         header_nodes.push(node);
      }
      let mut header = Node::with_children(header_size, header_nodes);
      header.move_to(Point::new(1.0, 1.0));

      let mut body_nodes = Vec::with_capacity(self.data.len());
      let mut table_size = Size::new(0.0, header_size.height);
      self.data.iter().for_each(|record| {
         let mut record_nodes = Vec::with_capacity(self.columns.len());
         let mut record_size = Size::ZERO;
         self
            .columns
            .iter()
            .map(|c| c.name.as_str())
            .map(|name| record.get_field_value(name))
            .filter_map(|h| h.ok())
            .for_each(|value| {
               let (width, height) = renderer.measure(&serde_json::to_string(&value).unwrap(), text_size, self.font, bounds);
               let size = {
                  let intrinsic = Size::new(width.min(self.column_max_width as f32), height);
                  limits.resolve(intrinsic).pad(padding)
               };
               let mut cell = Node::new(size);
               cell.move_to(Point::new(record_size.width, table_size.height));
               record_size.width += size.width;
               record_size.height = record_size.height.max(size.height);
               record_nodes.push(cell);
            });
         let record = Node::with_children(record_size, record_nodes);
         // record.move_to(Point::new(0.0, table_size.height));
         table_size.width = table_size.width.max(record_size.width);
         table_size.height += record_size.height;
         body_nodes.push(record);
      });
      let body = Node::with_children(table_size, body_nodes);
      // body.move_to(Point::new(0.0, header_size.height));
      table_size.width = header_size.width.max(table_size.width)+2.0;

      let mut divider = Node::new(Size::new(table_size.width, 1.0));
      divider.move_to(Point::new(0.0, header_size.height));
      Node::with_children(table_size, vec![header, divider, body])
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
         &self.state.orders,
         &self.data,
         self.is_orderable(),
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
         .map(|column| (column.to_string(), &column.name))
         .zip(self.data.clone())
         .for_each(|((label, name), record)| {
            label.hash(state);
            format!("{}", record.get_field_value(&name).unwrap()).hash(state);
         });
   }

   fn on_event(&mut self, event: Event, layout: Layout<'_>, cursor_position: Point, _messages: &mut Vec<Message>, _renderer: &Renderer, _clipboard: Option<&dyn Clipboard>) -> event::Status {
      let mouse_over = layout.bounds().contains(cursor_position);
      let mut children = layout.children();
      let header_layout = children.next().unwrap();
      let body_layout = children.next().unwrap();
      let mut event_status = event::Status::Ignored;

      if mouse_over {
         event_status = event::Status::Captured;
         match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
               if header_layout.bounds().contains(cursor_position) && self.is_orderable() {
                  let idx = self.columns.iter().enumerate().zip(header_layout.children()).find(|((_, _), layout)| layout.bounds().contains(cursor_position))
                     .map(|((idx, _), _)| idx).unwrap();
                  self.trigger_sort_column(idx);
               } else if body_layout.bounds().contains(cursor_position) {
                  self.selected_row = self.data.iter().enumerate().zip(body_layout.children())
                     .find(|((_, _), layout)| layout.bounds().contains(cursor_position)).map(|((idx, _), _)| idx);
               }
            }
            _ => {}
         }
      }

      event_status
   }
}

pub trait Renderer: text::Renderer {
   type Style: Default;
   const DEFAULT_PADDING: u16;
   const DEFAULT_HEADER_SPACING: u16;
   const DEFAULT_COL_MAX_WIDTH: u32 = 127;

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
      orders: &[TableOrder],
      data: &[T],
      is_orderable: bool,
      text_size: u16,
      padding: u16,
      font: Self::Font,
      style: &Self::Style,
   ) -> Self::Output;
}

impl Renderer for iced_wgpu::Renderer
where
{
   type Style = Box<dyn StyleSheet>;
   const DEFAULT_PADDING: u16 = 6;
   const DEFAULT_HEADER_SPACING: u16 = 1;
   const DEFAULT_COL_MAX_WIDTH: u32 = 127;

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
      orders: &[TableOrder],
      data: &[T],
      is_orderable: bool,
      text_size: u16,
      padding: u16,
      font: Self::Font,
      style: &Self::Style,
   ) -> Self::Output {
      let mut children = layout.children();
      let header_layout = children.next().unwrap();
      let divider_bounds = children.next().unwrap().bounds();
      let body_layout = children.next().unwrap();
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
      let divider = Primitive::Quad {
         bounds: divider_bounds,
         background: Background::Color(Color::TRANSPARENT),
         border_color: styling.border_color,
         border_width: styling.border_width,
         border_radius: 0.0,
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
            .enumerate()
            .zip(header_layout.children())
            .map(|((idx, content), layout)| {
               let bounds = layout.bounds();
               let formatted_label = match orders[idx] {
                  TableOrder::Unordered => content,
                  TableOrder::Ascending => format!("{} ▲", content),
                  TableOrder::Descending => format!("{} ▼", content),
               };
               Primitive::Text {
                  content: formatted_label,
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
      for ((idx, record), record_layout) in data.iter().enumerate().zip(body_layout.children()) {
         // let record_bg = if idx%2==0 {
         //    Primitive::Quad {
         //       bounds: record_layout.bounds(),
         //       background: styling.header_background,
         //       border_color: styling.border_color,
         //       border_radius: styling.border_radius,
         //       border_width: styling.border_width
         //    } 
         // } else {
         //    Primitive::None
         // };
         let record_cells = columns
            .iter()
            .map(|c| c.name.as_str())
            .map(|name| record.get_field_value(name))
            .filter_map(|h| h.ok())
            .zip(record_layout.children())
            .fold(Vec::with_capacity(columns.len()), |mut record_cells, (value, cell_layout)| {
               let bounds = cell_layout.bounds();
               let text = Primitive::Text {
                  content: serde_json::to_string(&value).unwrap(),
                  // content: serde_json::from_value(value).unwrap(),
                  size: f32::from(text_size),
                  font,
                  color: styling.text_color,
                  bounds: Rectangle {
                     x: bounds.x + f32::from(padding),
                     y: bounds.center_y(),
                     ..bounds
                  },
                  horizontal_alignment: HorizontalAlignment::Left,
                  vertical_alignment: VerticalAlignment::Top,
               };
               record_cells.push(
                  Primitive::Clip {
                     bounds,
                     content: Box::new(text),
                     offset: Vector::new(0, 0)
                  }
               );
               record_cells
            });
         let record = Primitive::Group{ primitives: record_cells };
         body_records.push(Primitive::Group{ primitives: vec![record] });
      }
      let body_section = Primitive::Group{ primitives: body_records };
      (
         Primitive::Group{ primitives: vec![background, header_background, body_section, divider, header_section] },
         if header_mouse_over && is_orderable {
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