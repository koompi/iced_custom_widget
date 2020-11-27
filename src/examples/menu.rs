use super::app::{App, AppMessage};
use crate::components::grid::Grid;
use crate::components::stepper::{self, Stepper};
use crate::styles::custom_styles::CustomTextInput;
use crate::utils::themes::Theme;
use iced::{
   button, executor, scrollable, text_input, window, Align, Application, Color, Column, Command, Container, Element, Length,
   Row, Scrollable, Settings, TextInput,
};
pub struct Menu {
   input_search: text_input::State,
   search_text: String,
   applications: Vec<App>,
   filtered_application: Vec<App>,
   scroll: scrollable::State,
   scale_state: ScaleState,
}

pub struct ScaleState {
   scale: f32,
   decrease_btn_state: stepper::State,
   increase_btn_state: stepper::State,
}

impl Menu {
   pub fn init() {
      let settings = Settings {
         default_text_size: 13,
         antialiasing: true,
         window: window::Settings {
            resizable: true,
            min_size: Some((410, 600)),
            always_on_top: true,
            transparent: true,
            decorations: true,
            ..window::Settings::default()
         },
         ..Settings::default()
      };
      Menu::run(settings).unwrap();
   }
}
#[derive(Debug, Clone)]
pub enum MenuMessage {
   SearchChanged(String),
   ActionSearch,
   AppMessage(usize, AppMessage),
   ScaleChanged(f32),
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
            icon: '\u{f3aa}',
            name: "Google Drive".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f3ab}',
            name: "Play Store".to_string(),
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
         App {
            icon: '\u{f181}',
            name: "Trello".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f099}',
            name: "Twitter".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f402}',
            name: "Uber".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f403}',
            name: "UIKit".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f189}',
            name: "VK".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f232}',
            name: "Whatsapp".to_string(),
            clickable: button::State::new(),
         },
         App {
            icon: '\u{f19e}',
            name: "Yahoo".to_string(),
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
            scale_state: ScaleState {
               scale: 1.0,
               decrease_btn_state: stepper::State::default(),
               increase_btn_state: stepper::State::default(),
            },
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
               .filter(|app| app.name.to_lowercase().contains(&self.search_text.to_lowercase()))
               .cloned()
               .collect();

            Command::none()
         }
         MenuMessage::ActionSearch => {
            self.filtered_application = self
               .applications
               .iter()
               .filter(|app| app.name.to_lowercase().contains(&self.search_text.to_lowercase()))
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
         MenuMessage::ScaleChanged(scale) => {
            self.scale_state.scale = scale;
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
      .style(CustomTextInput::Default(Theme::light().palette))
      .on_submit(MenuMessage::ActionSearch);
      let resizer = Stepper::new(
         self.scale_state.scale,
         &mut self.scale_state.decrease_btn_state,
         &mut self.scale_state.increase_btn_state,
         MenuMessage::ScaleChanged,
      )
      .step(0.1)
      .min(0.5)
      .max(2.);
      let search_section = Container::new(search).center_x().center_y().width(Length::Fill);
      let search_bar = Row::new()
         .spacing(20)
         .align_items(Align::Center)
         .push(search_section)
         .push(resizer);

      let menu: Element<_> = self
         .filtered_application
         .iter_mut()
         .enumerate()
         .fold(Grid::new().column_width(175), |grid, (i, app)| {
            grid.push(app.view().map(move |message| MenuMessage::AppMessage(i, message)))
         })
         .into();

      let content = Column::new()
         .spacing(20)
         .align_items(Align::Center)
         .push(search_bar)
         .push(menu);

      Scrollable::new(&mut self.scroll)
         .padding(30)
         .push(Container::new(content).width(Length::Fill).center_y().center_x())
         .into()
   }

   fn mode(&self) -> window::Mode {
      window::Mode::Fullscreen
   }

   fn background_color(&self) -> Color {
      Color::from_rgba8(255, 255, 255, 0.3)
   }

   fn scale_factor(&self) -> f64 {
      self.scale_state.scale as f64
   }
}
