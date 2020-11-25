use crate::components::table::{Table, TableData, TableError, TableResult};
use iced::{window, Container, Element, Length, Sandbox, Settings, Color, scrollable, Scrollable};
use rand::Rng;
use serde::Serialize;
use serde_value::Value;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Task {
   pub id: String,
   pub description: String,
   pub responsible_user: Option<String>,
   pub progress: i8,
   pub is_locked: bool,
   pub is_archived: bool,
   pub is_favorite: bool,
}

impl TableData for Task {
   fn get_field_value(&self, field_name: &str) -> TableResult<Value> {
      let value = match field_name {
         "id" => serde_value::to_value(
            &self
               .id
               .chars()
               .skip(5)
               .collect::<String>() // omit prefix "task-"
               .parse::<i32>()
               .unwrap(), // parse the number as integer
         ),
         "description" => serde_value::to_value(&self.description),
         "responsible_user" => serde_value::to_value(&self.responsible_user),
         "progress" => serde_value::to_value(self.progress),
         "is_locked" => serde_value::to_value(self.is_locked),
         "is_favorite" => serde_value::to_value(self.is_favorite),
         "is_archived" => serde_value::to_value(self.is_archived),
         s => return Err(TableError::InvalidFieldName(s.to_owned())),
      };
      Ok(value.unwrap())
   }
}

fn create_mock_tasks() -> Vec<Task> {
   let mut rng = rand::thread_rng();
   (0..10)
      .map(|i| Task {
         id: format!("task-{}", i + 1),
         description: String::from("These are not the Lorem Ipsums you are looking for"),
         is_favorite: rng.gen(),
         is_archived: rng.gen(),
         ..Task::default()
      })
      .collect()
}

pub struct TableDemo {
   scrollable_state: scrollable::State,
   tasks: Vec<Task>,
}

impl TableDemo {
   pub fn init() {
      let settings = Settings {
         default_font: Some(include_bytes!("../../fonts/ProductSans-Regular.ttf")),
         antialiasing: true,
         window: window::Settings {
            resizable: true,
            transparent: true,
            decorations: true,
            ..window::Settings::default()
         },
         ..Settings::default()
      };
      TableDemo::run(settings).unwrap();
   }
}

impl Sandbox for TableDemo {
   type Message = ();

   fn new() -> Self {
      Self {
         tasks: create_mock_tasks(),
         scrollable_state: scrollable::State::default(),
      }
   }

   fn title(&self) -> String {
      String::from("Table Demo")
   }

   fn update(&mut self, _message: Self::Message) {
      todo!()
   }

   fn view(&mut self) -> Element<'_, Self::Message> {
      let columns = table_columns![
         ("id", "Id."),
         ("description", "Description"),
         ("progress", "Progress"),
         ("responsible_user", "User"),
         ("is_locked", "Locked", "Loc."),
         ("is_favorite", "Favorite", "Fav."),
         ("is_archived", "Archived", "Arch."),
      ];

      let table: Element<_> = Table::new(columns, &mut self.tasks).into();
      Scrollable::new(&mut self.scrollable_state)
         .push(
            Container::new(table.explain(Color::BLACK))
               .center_x()
               .center_y()
               .width(Length::Fill),
         )
         .into()
   }
}
