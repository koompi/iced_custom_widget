// use iced_custom_widget as icw;
// use std::iter::Iterator;
// use iced::{scrollable, window, Color, Element, Length, Sandbox, Scrollable, Settings};
// use icw::components::{table_header,
//     table_header::TableHeader,
//     table_column, 
//     table_column::TableColumn, 
//  table_header::ResizeEvent};
// use icw::components::table_bk::table::State;
// pub struct TableDemo {
//     table_state: State<ColumnField>,
//     header_state: table_header::State,
// }

// impl TableDemo {
//     pub fn init() {
//         let setting = Settings {
//             default_text_size: 13,
//             antialiasing: true,
//             window: window::Settings {
//                 resizable: true,
//                 size: (500, 300),
//                 transparent: true,
//                 decorations: true,
//                 ..window::Settings::default()
//             },
//             ..Settings::default()
//         };
//         TableDemo::run(setting).unwrap();
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     ColumnResize(ResizeEvent),
//     ColumnPressed,
// }

// impl Default for TableDemo {
//     fn default() -> Self {
//         Self {
//             header_state: table_header::State::default(),
//             table_state: State {
//                 scrollable_state: scrollable::State::default(),
//                 column_key: None,
//                 sort_direction: None,
//                 columns: vec![
//                     table_column::State {
//                         key: ColumnField::Title,
//                         is_pressed: false,
//                         width: Length::FillPortion(2),
//                     },
//                     table_column::State {
//                         key: ColumnField::Author,
//                         is_pressed: false,
//                         width: Length::FillPortion(2),
//                     },
//                     table_column::State {
//                         key: ColumnField::Category,
//                         is_pressed: false,
//                         width: Length::FillPortion(3),
//                     },
//                     table_column::State {
//                         key: ColumnField::Location,
//                         is_pressed: false,
//                         width: Length::FillPortion(3),
//                     },
//                     table_column::State {
//                         key: ColumnField::Status,
//                         is_pressed: false,
//                         width: Length::Units(100),
//                     },
//                 ],
//             },
//         }
//     }
// }

// #[derive(Debug, Clone, Copy , PartialEq, std::hash::Hash, Eq)]
// pub enum ColumnField {
//     Title,
//     Author,
//     Category,
//     Location,
//     Status,
// }

// impl Default for ColumnField {
//     fn default() -> Self {
//         Self::Title
//     }
// }

// impl<'a> From<&'a str> for ColumnField {
//     fn from(s: &'a str) -> Self {
//         use ColumnField::*;
//         match s {
//             "title" => Title,
//             "author" => Author,
//             "category" => Category,
//             "location" => Location,
//             "status" => Status,
//             _ => panic!(format!("Can not found Column from {}", s)),
//         }
//     }
// }

// impl table_column::ColumnKey for ColumnField {
//     fn all() -> Vec<ColumnField> {
//         vec![
//             ColumnField::Title,
//             ColumnField::Author,
//             ColumnField::Category,
//             ColumnField::Location,
//             ColumnField::Status,
//         ]
//     }

//     fn title(self) -> String {
//         use ColumnField::*;
//         let title = match self {
//             Title => "Title",
//             Author => "Author",
//             Category => "Category",
//             Location => "Location",
//             Status => "Status",
//         };

//         title.to_owned()
//     }

//     fn as_string(self) -> String {
//         use ColumnField::*;
//         let s = match self {
//             Title => "title",
//             Author => "author",
//             Category => "category",
//             Location => "location",
//             Status => "status",
//         };

//         s.to_owned()
//     }
// }

// impl Sandbox for TableDemo {
//     type Message = Message;

//     fn new() -> Self {
//         Self::default()
//     }

//     fn title(&self) -> String {
//         String::from("Table Widget")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::ColumnResize(resize_event) => match resize_event {
//                 ResizeEvent::ResizeColumn {
//                     left_name,
//                     left_width,
//                     right_name,
//                     right_width,
//                 } => {
//                     let left_key = ColumnField::from(left_name.as_str());
//                     let right_key = ColumnField::from(right_name.as_str());

//                     println!(
//                         "{:?} {} {:?} {}",
//                         left_key, left_width, right_key, right_width
//                     );

//                     if let Some(column) = self
//                         .table_state
//                         .columns
//                         .iter_mut()
//                         .find(|c| c.key == left_key && left_key != ColumnField::Title)
//                     {
//                         column.width = Length::Units(left_width);
//                     }

//                     if let Some(column) = self
//                         .table_state
//                         .columns
//                         .iter_mut()
//                         .find(|c| c.key == right_key && right_key != ColumnField::Title)
//                     {
//                         column.width = Length::Units(right_width);
//                     }
//                 }
//                 ResizeEvent::Finished => println!("Resize Finished"),
//             },
//             Message::ColumnPressed => println!("Column Pressed"),
//         }
//     }

//     fn view(&mut self) -> Element<'_, Message> {
//         let mut columns = Vec::with_capacity(ColumnField::all().len());
//         for (field, column_state) in ColumnField::all().iter().zip(&mut self.table_state.columns) {
//             let name = field.as_string();
//             let table_column = TableColumn::new(column_state);
//             columns.push((name, table_column));
//         }
//         let header: Element<_> = TableHeader::new(&mut self.header_state, columns)
//             .width(Length::Fill)
//             .on_resize(3, Message::ColumnResize)
//             .into();

//         header.explain(Color::BLACK)
//         // Scrollable::new(&mut self.table_state.scrollable_state)
//         //     .push()
//         //     .into()
//         // Table::new(&mut self.table_state, columns)
//         //     .into()
//     }
// }

fn main() {
    println!("table_demo.bk");
}
