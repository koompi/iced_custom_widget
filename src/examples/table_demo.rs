use crate::components::table::{Table, TableData, TableError, TableResult, TableOptions};
use iced::{window, Container, Element, Length, Sandbox, Settings, scrollable, Scrollable};
use rand::Rng;
use crate::styles::custom_styles::CustomScrollable;
use crate::utils::themes::Theme;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Task {
   pub id: String,
   pub description: String,
   pub responsible_user: Option<String>,
   pub progress: u8,
   pub is_locked: bool,
   pub is_archived: bool,
   pub is_favorite: bool,
}

impl TableData for Task {
   fn get_field_value(&self, field_name: &str) -> TableResult<Value> {
      let value = match field_name {
         "id" => serde_json::to_value(
            &self
               .id
               .chars()
               .skip(5)
               .collect::<String>() // omit prefix "task-"
               .parse::<i32>()
               .unwrap(), // parse the number as integer
         ),
         "description" => serde_json::to_value(&self.description),
         "responsible_user" => serde_json::to_value(&self.responsible_user),
         "progress" => serde_json::to_value(&self.progress),
         "is_locked" => serde_json::to_value(&self.is_locked),
         "is_favorite" => serde_json::to_value(&self.is_favorite),
         "is_archived" => serde_json::to_value(&self.is_archived),
         s => return Err(TableError::InvalidFieldName(s.to_owned())),
      };
      Ok(value.unwrap())
   }
}

fn create_mock_tasks() -> Vec<Task> {
   let mut rng = rand::thread_rng();
   (0..50)
      .map(|i| Task {
         id: format!("task-{}", i + 1),
         description: String::from("These are not the Lorem Ipsums you are looking for"),
         progress: rng.gen::<u8>().min(100),
         is_favorite: rng.gen(),
         is_archived: rng.gen(),
         is_locked: rng.gen(),
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
         antialiasing: true,
         default_text_size: 13,
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
      let tasks = create_mock_tasks();
      Self {
         scrollable_state: scrollable::State::default(),
         tasks,
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
         ("id", "ID"),
         ("description", "Description"),
         ("progress", "Progress"),
         ("responsible_user", "User"),
         ("is_locked", "Locked", "Loc."),
         ("is_favorite", "Favorite", "Fav."),
         ("is_archived", "Archived", "Arch."),
      ];
      let option = TableOptions{ orderable: true };
      let table = Table::new(columns, &mut self.tasks).option(option).column_max_width(227);
      Scrollable::new(&mut self.scrollable_state)
         .push(
            Container::new(table)
               .center_x()
               .center_y()
               .width(Length::Fill),
         )
         .style(CustomScrollable::Default(Theme::light().palette))
         .into()
   }
}
