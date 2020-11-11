use super::app::{App, AppMessage};
use crate::components::grid::Grid;
use crate::styles::custom_text_input::CustomTextInput;
use iced::{
    button, executor, scrollable, text_input, Align, Application, Color, Column, Command,
    Container, Element, Length, Scrollable, Settings, TextInput,
};
// use crate::components::k_button;

pub struct Menu {
    input_search: text_input::State,
    search_text: String,
    applications: Vec<App>,
    filtered_application: Vec<App>,
    scroll: scrollable::State,
}

impl Menu {
    pub fn init() -> iced::Result {
        Menu::run(Settings::default())
    }
}
#[derive(Debug, Clone)]
pub enum MenuMessage {
    SearchChanged(String),
    ActionSearch,
    AppMessage(usize, AppMessage),
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = MenuMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let applications = vec![
            App {
                icon: '\u{f834}',
                name: "Airbnb".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f36f}',
                name: "App Store".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f375}',
                name: "AWS".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f268}',
                name: "Google Chrom".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f1cb}',
                name: "CodePen".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f391}',
                name: "Digital Ocean".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f392}',
                name: "Discord".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f395}',
                name: "Docker".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f09a}',
                name: "Facebook".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f39f}',
                name: "Messenger".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f269}',
                name: "Firefox".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f09b}',
                name: "Github".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f16d}',
                name: "Instagram".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f3b5}',
                name: "Music".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f08c}',
                name: "Linkedin".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f267}',
                name: "Safari".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f2c6}',
                name: "Telegram".to_string(),
                clickable: button::State::new(),
            },
            App {
                icon: '\u{f16c}',
                name: "StackOverflow".to_string(),
                clickable: button::State::new(),
            },
        ];
        (
            Self {
                input_search: text_input::State::new(),
                search_text: String::new(),
                applications: applications.clone(),
                filtered_application: applications.clone(),
                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Menu")
    }

    fn update(&mut self, message: MenuMessage) -> Command<Self::Message> {
        match message {
            MenuMessage::SearchChanged(text) => {
                self.search_text = text;
                self.filtered_application = self
                    .applications
                    .iter()
                    .filter(|app| {
                        app.name
                            .to_lowercase()
                            .contains(&self.search_text.to_lowercase())
                    })
                    .cloned()
                    .collect();

                Command::none()
            }
            MenuMessage::ActionSearch => {
                self.filtered_application = self
                    .applications
                    .iter()
                    .filter(|app| {
                        app.name
                            .to_lowercase()
                            .contains(&self.search_text.to_lowercase())
                    })
                    .cloned()
                    .collect();

                Command::none()
            }
            MenuMessage::AppMessage(i, app_message) => {
                if let Some(app) = self.applications.get_mut(i) {
                    app.update(app_message);
                }

                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let search = TextInput::new(
            &mut self.input_search,
            "Search",
            &mut self.search_text,
            MenuMessage::SearchChanged,
        )
        .padding(10)
        .max_width(800)
        .width(Length::Units(500))
        .size(17)
        .style(CustomTextInput::Default)
        .on_submit(MenuMessage::ActionSearch);
        let search_section = Container::new(search).center_x().center_y();

        // let search_section = Row::new()
        //     .push(Space::with_width(Length::FillPortion(2)))
        //     .push(Container::new(search).width(Length::FillPortion(3)))
        //     .push(Space::with_width(Length::FillPortion(2)));

        let menu: Element<_> = self
            .filtered_application
            .iter_mut()
            .enumerate()
            .fold(Grid::new().column_width(175), |grid, (i, app)| {
                grid.push(
                    app.view()
                        .explain(Color::BLACK)
                        .map(move |message| MenuMessage::AppMessage(i, message)),
                )
            })
            .into();

        let content = Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(search_section)
            .push(menu.explain(Color::BLACK));

        Scrollable::new(&mut self.scroll)
            .padding(30)
            .push(
                Container::new(content)
                    .width(Length::Fill)
                    .center_y()
                    .center_x(),
            )
            .into()
    }
}
