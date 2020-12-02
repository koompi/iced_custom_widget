use super::pref::{Pref, PrefMessage, Category};
use super::styles::CustomButton;
use crate::components::{
   grid::Grid,
   icon::Icon
};
use crate::styles::custom_styles::{CustomTextInput, CustomContainer};
use crate::utils::themes::Theme;
use iced::{
   executor, scrollable, text_input, button, Align, Application, Column, Command, 
   Container, Element, Length, Row, Scrollable, TextInput, Settings, Space, Text, Button
};

pub struct SystemSetting {
   input_search: text_input::State,
   search_text: String,
   prefs: Vec<Pref>,
   selected_pref: Option<usize>,
   pages_stack: Vec<PageModel>,
   back_btn_state: button::State,
   sidebar_scroll: scrollable::State,
   scroll: scrollable::State,
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
            pages_stack: vec![PageModel::HomePage],
            selected_pref: None,
            back_btn_state: button::State::new(),
            sidebar_scroll: scrollable::State::new(),
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

            // Command::perform(future, SystemMessage::Navigation())
         },
         Self::Message::Navigation(page) => {
            let next_page = match page {
               Page::HomePage => PageModel::HomePage
            };
            self.pages_stack.push(next_page);
         },
         Self::Message::NavigateBack => {
            if self.pages_stack.len() > 1 {
               self.pages_stack.pop();
            }
         }
      }
      Command::none()
   }

   fn view(&mut self) -> Element<Self::Message> {
      let search = TextInput::new(&mut self.input_search, "Search", &mut self.search_text, Self::Message::SearchChanged)
         .padding(10)
         .max_width(800)
         .width(Length::Units(500))
         .size(17)
         .style(CustomTextInput::Default(Theme::light().palette))
         .on_submit(Self::Message::ActionSearch);
      let search_section = Container::new(search).center_x().center_y().width(Length::Fill);
      let mut back_btn = Button::new(&mut self.back_btn_state, Icon::new('\u{f104}').size(20))
         .padding(7)
         .style(CustomButton::Default);
      if self.pages_stack.len() > 1 {
         back_btn = back_btn.on_press(SystemMessage::NavigateBack);
      }
      let search_bar = Row::new()
         .push(back_btn)
         .push(search_section);

      let sidebar = if let Some(selected_pref) = &self.selected_pref {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
            .fold((Column::new().spacing(10), Column::new().spacing(10)), |(personal_prefs, device_prefs), (idx, pref)| {
               match pref.category {
                  Category::Personal => (personal_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
                  Category::Device => (personal_prefs, device_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message))))
               }
            });
         let personal_section = Column::new()
            .spacing(15)
            .push(Container::new(Text::new("Personal").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            .push(personal_prefs);
         let device_section = Column::new()
            .spacing(15)
            .push(Container::new(Text::new("Device").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            .push(device_prefs);
         Container::new(
            Scrollable::new(&mut self.sidebar_scroll)
            .padding(15)
            .spacing(20)
            .scroller_width(5)
            .scrollbar_width(5)
            .push(personal_section)
            .push(device_section)
         )
         .width(Length::Units(125))
      } else {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
         .fold((Grid::new().column_width(125), Grid::new().column_width(125)), |(personal_prefs, device_prefs), (idx, pref)| {
            match pref.category {
               Category::Personal => (personal_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
               Category::Device => (personal_prefs, device_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message)))),
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
            Scrollable::new(&mut self.scroll)
            .push(
               Column::new()
               .spacing(30)
               .width(Length::Fill)
               .align_items(Align::Center)
               .push(personal_section)
               .push(device_section)
            )
         ).width(Length::Fill)
      };

      let content = match self.pages_stack.last().unwrap() {
         PageModel::HomePage => Container::new(Space::with_width(Length::Shrink))
      };

      Container::new(
         Column::new()
         .spacing(30)
         .width(Length::Fill)
         .align_items(Align::Center)
         .push(search_bar)
         .push(
            Row::new()
            .spacing(20)
            .push(sidebar)
            .push(content)
         )
      ).padding(30).into()
   }
}

#[derive(Debug, Clone)]
pub enum Page {
   HomePage,
   // GeneralPage
}

#[derive(Debug, Clone)]
enum PageModel {
   HomePage,
   // GeneralPage {

   // }
}

impl SystemSetting {
   pub fn init() -> iced::Result {
      SystemSetting::run(Settings {
         default_text_size: 13,
         ..Settings::default()
      })
   }
}