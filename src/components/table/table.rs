use super::{
   error::Result,
   table_column::{TableColumn, TableOptions, TableOrder},
};
use crate::styles::table::StyleSheet;
use iced_graphics::Primitive;
use iced_native::{
   event::{self, Event}, layout::{Limits, Node}, mouse, text, scrollable, container, 
   Color, Element, Hasher, Clipboard, Vector, Point, Rectangle, Length, Size,
   HorizontalAlignment, Layout, VerticalAlignment, Widget, Background, Container, Scrollable,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub trait TableData: 'static + Default + Clone + Ord + Serialize + DeserializeOwned {
   fn get_field_value(&self, field_name: &str) -> Result<Value>;
}

#[derive(Debug, Clone, Default)]
pub struct State {
   scrollable: scrollable::State,
}
impl State {
   pub fn new() -> Self {
      Self::default()
   }
}

pub struct Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   state: &'a mut State,
   columns: Vec<TableColumn>,
   data: &'a mut Vec<T>,
   option: Option<TableOptions>,
   selected_row: Option<usize>,
   width: Length,
   padding: u16,
   header_spacing: u16,
   column_max_width: Option<f32>,
   text_size: Option<u16>,
   font: Renderer::Font,
   style: Renderer::Style,
}

impl<'a, T, Renderer> Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   pub fn new(state: &'a mut State, columns: Vec<TableColumn>, data: &'a mut Vec<T>) -> Self {
      Table {
         state,
         columns,
         data,
         option: None,
         selected_row: None,
         width: Length::Shrink,
         padding: Renderer::DEFAULT_PADDING,
         header_spacing: Renderer::DEFAULT_HEADER_SPACING,
         column_max_width: None,
         text_size: None,
         font: Renderer::Font::default(),
         style: Renderer::Style::default(),
      }
   }

   pub fn option(mut self, option: TableOptions) -> Self {
      self.option = Some(option);
      self
   }

   pub fn width(mut self, width: Length) -> Self {
      self.width = width;
      self
   }

   pub fn padding(mut self, padding: u16) -> Self {
      self.padding = padding;
      self
   }

   pub fn column_max_width(mut self, column_max_width: f32) -> Self {
      self.column_max_width = Some(column_max_width);
      self
   }

   pub fn header_spacing(mut self, spacing: u16) -> Self {
      self.header_spacing = spacing;
      self
   }

   pub fn text_size(mut self, text_size: u16) -> Self {
      self.text_size = Some(text_size);
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
         true
      }
   }

   fn trigger_sort_column(&mut self, idx: usize) {
      use TableOrder::*;

      for (i, column) in self.columns.iter_mut().enumerate() {
         let TableColumn{name, order, ..} = column;
         if i != idx {
            *order = Unordered;
         } else {
            *order = order.toggle();
            match order {
               Unordered => self.data.sort(),
               Ascending => self.data.sort_by_cached_key(|x| x.get_field_value(&name).unwrap().to_string()),
               Descending => self.data.sort_by_cached_key(|x| std::cmp::Reverse(x.get_field_value(&name).unwrap().to_string())),
            }
         }
      }
   }
}

impl<'a, Message, Renderer, T> Widget<Message, Renderer> for Table<'a, T, Renderer>
where
   T: TableData,
   Renderer: self::Renderer,
{
   fn width(&self) -> Length {
      self.width
   }

   fn height(&self) -> Length {
      Length::Shrink
   }

   fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
      let padding = f32::from(self.padding);
      let text_size = self.text_size.unwrap_or(renderer.default_size());
      let limits = limits.width(self.width).height(Length::Shrink);
      let mut max_cols_size: Vec<Size> = Vec::with_capacity(self.columns.len());

      for column in self.columns.iter() {
         let (width, height) = renderer.measure(&formatted_sortable_column(column.to_string(), column.order), text_size, self.font, Size::new(f32::INFINITY, f32::INFINITY),);
         let size = {
            let intrinsic = Size::new(width+f32::from(text_size), height);
            // limits.resolve(intrinsic).pad(padding)
            intrinsic.pad(padding)
         };
         max_cols_size.push(size);
      }
      for record in self.data.iter() {
         self.columns.iter()
         .map(|c| c.name.as_str())
         .map(|name| record.get_field_value(name))
         .filter_map(|h| h.ok())
         .enumerate()
         .for_each(|(idx, value)| {
            let (width, height) = renderer.measure(&serde_json::to_string(&value).unwrap(), text_size, self.font, Size::new(f32::INFINITY, f32::INFINITY),);
            let size = {
               let intrinsic = Size::new(if let Some(max_width) = self.column_max_width {width.min(max_width)} else {width}, height);
               // limits.resolve(intrinsic).pad(padding)
               intrinsic.pad(padding)
            };
            if let Some(max_size) = max_cols_size.get_mut(idx) {
               max_size.width = max_size.width.max(size.width);
               max_size.height = max_size.height.max(size.height);
            }
         });
      }

      let mut header_nodes = Vec::new();
      let mut header_size = Size::ZERO;
      for size in max_cols_size.iter() {
         let mut node = Node::new(*size);
         node.move_to(Point::new(header_size.width, 0.0));
         header_size.width += size.width;
         header_size.height = header_size.height.max(size.height);
         header_nodes.push(node);
      }
      let mut header = Node::with_children(header_size, header_nodes);
      header.move_to(Point::new(1.0, 1.0));

      let mut body_nodes = Vec::with_capacity(self.data.len());
      let mut table_size = Size::new(0.0, header_size.height);
      for _ in self.data.iter() {
         let mut record_nodes = Vec::with_capacity(self.columns.len());
         let mut record_size = Size::ZERO;
         for size in max_cols_size.iter() {
            let mut cell = Node::new(*size);
            cell.move_to(Point::new(record_size.width, table_size.height));
            record_size.width += size.width;
            record_size.height = record_size.height.max(size.height);
            record_nodes.push(cell);
         }
         let record = Node::with_children(record_size, record_nodes);
         // record.move_to(Point::new(1.0, table_size.height));
         table_size.width = table_size.width.max(record_size.width);
         table_size.height += record_size.height;
         body_nodes.push(record);
      }
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

// struct TableContainer<'a, Message, Renderer: self::Renderer> {
//    container: Container<'a, Message, Renderer>,
//    width: Length,
//    target_height: f32,
// }

// impl<'a, Message, Renderer> TableContainer<'a, Message, Renderer>
// where 
//    Message: 'a,
//    Renderer: 'a + self::Renderer
// {
//    pub fn new<T>(table: Table<'a, T, Renderer>, target_height: f32) -> Self
//    where T: TableData {
//       let Table {
//          state,
//          option,
//          width,
//          padding,
//          font,
//          text_size,
//          style,
//          ..
//       } = table;

//       let container = Container::new(Scrollable::new(&mut state.scrollable).push(child)).padding(1);
//       Self {
//          container,
//          width,
//          target_height,
//      }
//    }
// }

pub trait Renderer: text::Renderer {
   type Style: Default;
   const DEFAULT_PADDING: u16;
   const DEFAULT_HEADER_SPACING: u16;

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
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

   fn draw<T: TableData>(
      &mut self,
      defaults: &Self::Defaults,
      layout: Layout<'_>,
      cursor_position: Point,
      viewport: &Rectangle,
      columns: &[TableColumn],
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
            .zip(header_layout.children())
            .map(|(column, layout)| {
               let bounds = layout.bounds();
               Primitive::Text {
                  content: formatted_sortable_column(column.to_string(), column.order),
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
         // let record_bg = if idx%2==0 {
         //    Primitive::Quad {
         //       bounds: record_layout.bounds(),
         //       background: styling.header_background,
         //       border_color: Color::TRANSPARENT,
         //       border_radius: styling.border_radius,
         //       border_width: 0.0
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
                  vertical_alignment: VerticalAlignment::Center,
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
         body_records.push(record);
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

fn formatted_sortable_column(label: String, order: TableOrder) -> String {
   match order {
      TableOrder::Unordered => label,
      TableOrder::Ascending => format!("{} ▲", label),
      TableOrder::Descending => format!("{} ▼", label),
   }
}