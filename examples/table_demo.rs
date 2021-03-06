use iced::{scrollable, Container, Element, Length, Sandbox, Scrollable, Settings};
use iced_custom_widget as icw;
use icw::components::table::{self, Table, TableData, TableError, TableOptions, TableResult};
use icw::table_columns;
use rand::Rng;
use serde::{Deserialize, Serialize};
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
    (0..27)
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
    table_state: table::State,
}


fn main() -> iced::Result {
    TableDemo::run(Settings {
        default_text_size: 13,
        ..Settings::default()
    })
}

impl Sandbox for TableDemo {
    type Message = ();

    fn new() -> Self {
        let tasks = create_mock_tasks();
        Self {
            scrollable_state: Default::default(),
            tasks,
            table_state: Default::default(),
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
            ("is_locked", "Locked"),
            ("is_favorite", "Favorite"),
            ("is_archived", "Archived"),
        ];
        let option = TableOptions { orderable: true };
        let table = Table::new(&mut self.table_state, columns, &mut self.tasks)
            // .width(Length::Fill)
            // .column_max_width(227.0)
            .option(option);
        Scrollable::new(&mut self.scrollable_state)
            .push(
                Container::new(table)
                    .padding(20)
                    .width(Length::Fill)
                    .center_x()
                    .center_y(),
            )
            //  .style(CustomScrollable::Default(Theme::light().palette))
            .into()
    }
}
