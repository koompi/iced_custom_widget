use crate::styles::custom_button::CustomButton;
use iced::{button, Align, Button, Column, Element, Text, Svg, Length, HorizontalAlignment, Container};

#[derive(Debug, Clone)]
pub struct Pref {
   pub path: String,
   pub name: String,
   pub category: Category,
   pub button_state: button::State,
}

#[derive(Debug, Clone)]
pub enum Category {
   Personal,
   Device
}

#[derive(Debug, Clone, Copy)]
pub enum PrefMessage {
   PrefClicked,
}

impl Pref {
   pub fn new(path: String, name: String, category: Category) -> Self {
      Self {
         path,
         name,
         category,
         button_state: button::State::new(),
      }
   }

   pub fn update(&mut self, message: PrefMessage) {
      match message {
         PrefMessage::PrefClicked => {}
      }
   }

   pub fn view(&mut self) -> Element<PrefMessage> {
      let icon = Svg::from_path(&self.path).width(Length::Fill).height(Length::Fill);
      let icon_button = Button::new(&mut self.button_state, icon)
         .width(Length::Units(80))
         .height(Length::Units(80))
         .padding(10)
         .on_press(PrefMessage::PrefClicked)
         .style(CustomButton::Default);
      let name = Text::new(&self.name).horizontal_alignment(HorizontalAlignment::Center);
      let pref = Column::new().spacing(10).align_items(Align::Center).push(icon_button).push(name);
      Container::new(pref).width(Length::Units(100)).into()
   }
}
