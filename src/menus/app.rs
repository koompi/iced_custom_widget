use iced::{button, Button, Column, Container, Element, Font, HorizontalAlignment, Length, Text, Align, Color};
use super::custom_button::CustomButton;

const ICONS: Font = Font::External {
   name: "Line Awesome",
   bytes: include_bytes!("../../fonts/la-brands-400.ttf"),
};

fn icon(unicode: &char) -> Text {
   Text::new(unicode.to_string())
      .font(ICONS)
      .size(127)
      // .width(Length::Units(20))
      .horizontal_alignment(HorizontalAlignment::Center)
}

#[derive(Debug, Clone)]
pub struct App {
   pub icon: char,
   pub name: String,
   pub clickable: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
   AppClicked,
}

impl App {
   pub fn new(icon: char, name: String) -> Self {
      Self {
         icon,
         name,
         clickable: button::State::new(),
      }
   }

   pub fn update(&mut self, message: AppMessage) {
      match message {
         AppMessage::AppClicked => {
            self.icon = '\u{f09b}';
         }
      }
   }

   pub fn view(&mut self) -> Element<AppMessage> {
      let icon = icon(&self.icon);
      let name = Text::new(&self.name).size(15);
      let app = Column::new().push(icon).push(name).align_items(Align::Center);
      let app_btn = Button::new(&mut self.clickable, app)
         .padding(10)
         .min_height(75)
         .min_width(75)
         .style(CustomButton::Default)
         .on_press(AppMessage::AppClicked);

      Container::new(app_btn)
         .center_x()
         .center_y()
         .width(Length::Shrink)
         .height(Length::Shrink)
         .into()
   }
}
