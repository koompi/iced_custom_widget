use super::pref::{Pref, PrefMessage, Category};
use crate::components::grid::Grid;
use crate::styles::custom_styles::{CustomTextInput, CustomContainer};
use crate::utils::themes::Theme;
use iced::{
   executor, scrollable, text_input, window, button, Align, Application, Column, Command, 
   Container, Element, Length, Row, Scrollable, TextInput, Settings, Space, Text
};

pub struct SystemSetting {
   input_search: text_input::State,
   search_text: String,
   prefs: Vec<Pref>,
   selected_pref: Option<usize>,
   current_page: PageModel,
   previous_page: Option<PageModel>,
   back_btn_state: button::State,
   scroll: scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Page {
   HomePage,
}

enum PageModel {
   HomePage
}

impl SystemSetting {
   pub fn init() -> iced::Result {
      SystemSetting::run(Settings {
         default_text_size: 13,
         window: window::Settings {
            resizable: false,
            ..window::Settings::default()
         },
         ..Settings::default()
      })
   }
}

#[derive(Debug, Clone)]
pub enum SystemMessage {
   SearchChanged(String),
   ActionSearch,
   PrefMessage(usize, PrefMessage),
   Navigation(Page),
   NavigateBack
}

impl Application for SystemSetting {
   type Executor = executor::Default;
   type Message = SystemMessage;
   type Flags = ();

   fn new(_flags: ()) -> (Self, Command<Self::Message>) {
      use Category::*;
      let pref = |file_name: &str, name: &str, category: Category| {
         Pref::new(format!("{}/assets/images/{}.svg",env!("CARGO_MANIFEST_DIR"), file_name), String::from(name), category)
      };

      let prefs = vec![
         pref("window", "General", Personal),
         pref("time", "Date & Time", Personal),
         pref("language", "Language & Region", Personal),
         pref("users", "Users & Groups", Personal),
         pref("accessibility", "Accessibility", Personal),
         pref("account", "Accounts", Personal),
         pref("notification", "Notifications", Personal),
         pref("privacy", "Security & Privacy", Personal),
         pref("update", "Software Update", Personal),
         pref("network", "Network", Device),
         pref("bluetooth", "Bluetooth", Device),
         pref("sound", "Sound", Device),
         pref("printer", "Printers & Scanners", Device),
         pref("camera", "Camera", Device),
         pref("keyboard", "Keyboard", Device),
         pref("touchpad", "Touchpad", Device),
         pref("mic", "Microphone", Device),
         pref("mouse", "Mouse", Device),
         pref("display", "Display", Device),
         pref("battery", "Battery", Device),
         pref("disk", "Disk Drive", Device),
      ];

      (
         Self {
            input_search: text_input::State::new(),
            search_text: String::new(),
            prefs,
            current_page: PageModel::HomePage,
            previous_page: None,
            selected_pref: None,
            back_btn_state: button::State::new(),
            scroll: scrollable::State::new()
         },
         Command::none(),
      )
   }

   fn title(&self) -> String {
      String::from("Menu")
   }

   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
      match message {
         Self::Message::SearchChanged(text) => self.search_text = text,
         Self::Message::ActionSearch => println!("search submited"),
         Self::Message::PrefMessage(i, pref_message) => {
            if let Some(pref) = self.prefs.get_mut(i) {
               pref.update(pref_message);
               self.selected_pref = Some(i);
            }
         },
         Self::Message::Navigation(page) => {
            self.current_page = match page {
               Page::HomePage => PageModel::HomePage
            };
         },
         Self::Message::NavigateBack => {
            // if let Some(previous_page) = &self.previous_page {
            //    self.current_page = previous_page;
            // }
         }
      }
      Command::none()
   }

   fn view(&mut self) -> Element<Self::Message> {
      let search = TextInput::new(
         &mut self.input_search,
         "Search",
         &mut self.search_text,
         Self::Message::SearchChanged,
      )
      .padding(10)
      .max_width(800)
      .width(Length::Units(500))
      .size(17)
      .style(CustomTextInput::Default(Theme::light().palette))
      .on_submit(Self::Message::ActionSearch);
      let search_section = Container::new(search).center_x().center_y().width(Length::Fill);

      let content = if let Some(selected_pref) = &self.selected_pref {
         println!("{:?}", selected_pref);
         Container::new(
            Column::new()
               .spacing(30)
               .width(Length::Fill)
               .align_items(Align::Center)
               .push(search_section)
         )
      } else {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
         .fold((Grid::new().column_width(125), Grid::new().column_width(125)), |(personal_prefs, device_prefs), (idx, pref)| {
            match pref.category {
               Category::Personal => (personal_prefs.push(pref.view().map(move |message| Self::Message::PrefMessage(idx, message))), device_prefs),
               Category::Device => (personal_prefs, device_prefs.push(pref.view().map(move |message| Self::Message::PrefMessage(idx, message)))),
            }
         });

         let personal_section = Container::new(
            Column::new()
            .spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("Personal").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            )
            .push(personal_prefs)
         );
         let device_section = Container::new(
            Column::new()
            .spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("Device").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            )
            .push(device_prefs)
         );

         Container::new(
            Column::new()
            .spacing(30)
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(search_section)
            .push(personal_section)
            .push(device_section)
         )
      };

      Scrollable::new(&mut self.scroll).padding(20).push(content).into()
   }
}
