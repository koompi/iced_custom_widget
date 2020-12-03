use super::pref::{Pref, PrefMessage, Category};
use super::styles::CustomButton;
use super::pages::{Pages, PagesMessage};
use crate::components::{
   grid::Grid,
   icon::Icon
};
use crate::styles::custom_styles::{CustomTextInput, CustomContainer};
use crate::utils::themes::Theme;
use iced::{
   executor, scrollable, text_input, button, window, Align, Application, Column, Command, 
   Container, Element, Length, Row, Scrollable, TextInput, Settings, Space, Text, Button
};

pub struct SystemSetting {
   input_search: text_input::State,
   search_text: String,
   prefs: Vec<Pref>,
   selected_pref: Option<usize>,
   pages: Pages,
   back_btn_state: button::State,
   sidebar_scroll: scrollable::State,
   scroll: scrollable::State,
}


#[derive(Debug, Clone)]
pub enum SystemMessage {
   SearchChanged(String),
   ActionSearch,
   PrefMessage(usize, PrefMessage),
   PagesMessage(PagesMessage),
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
         pref("window", "General", System),
         pref("time", "Date & Time", System),
         pref("language", "Language & Region", System),
         pref("users", "Users & Groups", System),
         pref("accessibility", "Accessibility", System),
         pref("account", "Accounts", System),
         pref("notification", "Notifications", System),
         pref("privacy", "Security & Privacy", System),
         pref("update", "Software Update", System),
         pref("network", "Network", Hardware),
         pref("bluetooth", "Bluetooth", Hardware),
         pref("sound", "Sound", Hardware),
         pref("printer", "Printers & Scanners", Hardware),
         pref("camera", "Camera", Hardware),
         pref("keyboard", "Keyboard", Hardware),
         pref("touchpad", "Touchpad", Hardware),
         pref("mic", "Microphone", Hardware),
         pref("mouse", "Mouse", Hardware),
         pref("display", "Display", Hardware),
         pref("battery", "Battery", Hardware),
         pref("disk", "Disk Drive", Hardware),
      ];

      (
         Self {
            input_search: text_input::State::new(),
            search_text: String::new(),
            prefs,
            pages: Pages::new(),
            selected_pref: None,
            back_btn_state: button::State::new(),
            sidebar_scroll: scrollable::State::new(),
            scroll: scrollable::State::new()
         },
         Command::none(),
      )
   }

   fn title(&self) -> String {
      self.pages.title().to_string()
   }

   fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
      match message {
         Self::Message::SearchChanged(text) => self.search_text = text,
         Self::Message::ActionSearch => println!("search submited"),
         Self::Message::PrefMessage(i, pref_message) => {
            if let Some(pref) = self.prefs.get_mut(i) {
               pref.update(pref_message);
               self.selected_pref = Some(i);
               self.pages.set_current(i+1);
            }

            // Command::perform(future, SystemMessage::Navigation())
         },
         // Self::Message::Navigation(page) => {
         //    let next_page = match page {
         //       Page::HomePage => PageModel::HomePage
         //    };
         //    self.pages_stack.push(next_page);
         // },
         Self::Message::NavigateBack => {
            // if self.pages_stack.len() > 1 {
            //    self.pages_stack.pop();
            // }
            self.selected_pref = None;
            self.pages.set_current(0)
         }
         Self::Message::PagesMessage(page_msg) => {
            self.pages.update(page_msg);
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
      let mut back_btn = Button::new(&mut self.back_btn_state, Icon::new('\u{f0ce}').size(20))
         .padding(7)
         .style(CustomButton::Text);
      if self.selected_pref.is_some() {
         back_btn = back_btn.on_press(SystemMessage::NavigateBack);
      }
      let search_bar = Row::new()
         .spacing(20)
         .padding(30)
         .push(back_btn)
         .push(search_section);

      let sidebar = if let Some(selected_pref) = &self.selected_pref {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
            .fold((Column::new().spacing(10), Column::new().spacing(10)), |(personal_prefs, device_prefs), (idx, pref)| {
               match pref.category {
                  Category::System => (personal_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
                  Category::Hardware => (personal_prefs, device_prefs.push(pref.view_sidebar(idx == *selected_pref).map(move |message| SystemMessage::PrefMessage(idx, message))))
               }
            });
         let personal_section = Column::new()
            .spacing(15)
            .push(Container::new(Text::new("System").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            .push(personal_prefs);
         let device_section = Column::new()
            .spacing(15)
            .push(Container::new(Text::new("Hardware").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
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
         .width(Length::Units(110))
      } else {
         let (personal_prefs, device_prefs) = self.prefs.iter_mut().enumerate()
         .fold((Grid::new().column_width(125), Grid::new().column_width(125)), |(personal_prefs, device_prefs), (idx, pref)| {
            match pref.category {
               Category::System => (personal_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message))), device_prefs),
               Category::Hardware => (personal_prefs, device_prefs.push(pref.view_main().map(move |message| SystemMessage::PrefMessage(idx, message)))),
            }
         });
         
         let personal_section = Container::new(
            Column::new()
            .spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("System").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            )
            .push(personal_prefs)
         );
         let device_section = Container::new(
            Column::new()
            .spacing(15)
            .push(
               Row::new()
               .push(Space::with_width(Length::Units(20)))
               .push(Container::new(Text::new("Hardware").size(15)).padding(7).style(CustomContainer::FadedBrightForeground(Theme::light().palette)))
            )
            .push(device_prefs)
         );
         
         Container::new(
            Scrollable::new(&mut self.scroll)
            .padding(20)
            .push(
               Column::new()
               .spacing(30)
               .width(Length::Fill)
               .align_items(Align::Center)
               .push(personal_section)
               .push(device_section)
            )
         )
         .width(Length::Fill)
      };

      let content = self.pages.view().map(SystemMessage::PagesMessage);

      Container::new(
         Column::new()
         .spacing(10)
         .width(Length::Fill)
         .push(search_bar)
         .push(
            Row::new()
            .spacing(15)
            .push(sidebar)
            .push(content)
         )
      ).into()
   }
}

impl SystemSetting {
   pub fn init() -> iced::Result {
      SystemSetting::run(Settings {
         default_text_size: 13,
         window: window::Settings {
            min_size: Some((600, 700)),
            ..window::Settings::default()
         },
         ..Settings::default()
      })
   }
}