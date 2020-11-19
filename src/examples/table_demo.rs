use crate::components::table::{self, Table};
use crate::components::table_column::{self, ColumnKey};
use crate::components::table_header::{self, ResizeEvent, TableHeader};

use iced::{scrollable, window, Element, Length, Sandbox, Settings, Scrollable};

pub struct TableDemo {
    header_state: table_header::State<ColumnField>,
    scrollable_state: scrollable::State,
}

impl TableDemo {
    pub fn init() {
        let setting = Settings {
            default_text_size: 13,
            antialiasing: true,
            window: window::Settings {
                resizable: true,
                size: (500, 300),
                transparent: true,
                decorations: true,
                ..window::Settings::default()
            },
            ..Settings::default()
        };
        TableDemo::run(setting).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ColumnResize(ResizeEvent),
    ColumnPressed,
}

impl Default for TableDemo {
    fn default() -> Self {
        Self {
            scrollable_state: Default::default(),
            header_state: table_header::State {
                internal: Default::default(),
                previous_column_key: None,
                previous_sort_direction: None,
                columns: vec![
                    table_column::State {
                        key: ColumnField::Title,
                        is_pressed: false,
                        width: Length::FillPortion(2),
                        height: Length::Shrink,
                    },
                    table_column::State {
                        key: ColumnField::Author,
                        is_pressed: false,
                        width: Length::FillPortion(2),
                        height: Length::Shrink,
                    },
                    table_column::State {
                        key: ColumnField::Category,
                        is_pressed: false,
                        width: Length::FillPortion(3),
                        height: Length::Shrink,
                    },
                    table_column::State {
                        key: ColumnField::Location,
                        is_pressed: false,
                        width: Length::FillPortion(3),
                        height: Length::Shrink,
                    },
                    table_column::State {
                        key: ColumnField::Status,
                        is_pressed: false,
                        width: Length::Units(100),
                        height: Length::Shrink,
                    },
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, std::hash::Hash, Eq)]
pub enum ColumnField {
    Title,
    Author,
    Category,
    Location,
    Status,
}

impl Default for ColumnField {
    fn default() -> Self {
        Self::Title
    }
}

impl<'a> From<&'a str> for ColumnField {
    fn from(s: &'a str) -> Self {
        use ColumnField::*;
        match s {
            "title" => Title,
            "author" => Author,
            "category" => Category,
            "location" => Location,
            "status" => Status,
            _ => panic!(format!("Can not found Column from {}", s)),
        }
    }
}

impl ColumnKey for ColumnField {
    fn all() -> Vec<ColumnField> {
        vec![
            ColumnField::Title,
            ColumnField::Author,
            ColumnField::Category,
            ColumnField::Location,
            ColumnField::Status,
        ]
    }

    fn title(self) -> String {
        use ColumnField::*;
        let title = match self {
            Title => "Title",
            Author => "Author",
            Category => "Category",
            Location => "Location",
            Status => "Status",
        };

        title.to_owned()
    }

    fn as_string(self) -> String {
        use ColumnField::*;
        let s = match self {
            Title => "title",
            Author => "author",
            Category => "category",
            Location => "location",
            Status => "status",
        };

        s.to_owned()
    }
}

impl Sandbox for TableDemo {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Table Widget")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ColumnResize(resize_event) => match resize_event {
                ResizeEvent::ResizeColumn {
                    left_name,
                    left_width,
                    right_name,
                    right_width,
                } => {
                    let left_key = ColumnField::from(left_name.as_str());
                    let right_key = ColumnField::from(right_name.as_str());

                    println!(
                        "{:?} {} {:?} {}",
                        left_key, left_width, right_key, right_width
                    );

                    if let Some(column) = self
                        .header_state
                        .columns
                        .iter_mut()
                        .find(|c| c.key == left_key && left_key != ColumnField::Title)
                    {
                        column.width = Length::Units(left_width);
                    }

                    if let Some(column) = self
                        .header_state
                        .columns
                        .iter_mut()
                        .find(|c| c.key == right_key && right_key != ColumnField::Title)
                    {
                        column.width = Length::Units(right_width);
                    }
                }
                ResizeEvent::Finished => println!("Resize Finished"),
            },
            Message::ColumnPressed => println!("Column Pressed"),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        // let columns = ColumnField::all()
        //     .iter()
        //     .zip(&mut self.header_state.columns)
        //     .map(|(field, column_state)| {
        //         let name = field.as_string();
        //         let table_column =
        //             TableColumn::new(column_state).on_press(Self::Message::ColumnPressed);
        //         (name, table_column)
        //     })
        //     .collect();
        let header = TableHeader::new(&mut self.header_state)
            .width(Length::Fill)
            // .height(Length::Units(20))
            .on_resize(3, Self::Message::ColumnResize);
        Scrollable::new(&mut self.scrollable_state)
            .push(Table::new(header))
            .into()
    }
}
