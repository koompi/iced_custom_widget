use crate::components::card::{Card, State};
use iced::{Container, Sandbox, Settings, Text, Element, Length};

pub struct CardTest {
   pub card_state: State,
}

#[derive(Debug, Clone)]
pub enum CardMessage {
   OnCardPressed
}

impl CardTest {
   pub fn init() -> iced::Result {
      CardTest::run(Settings::default())
   }
}

impl Sandbox for CardTest {
   type Message = CardMessage;

   fn new() -> Self {
      Self {
         card_state: State::default()
      }
   }

   fn title(&self) -> String {
      String::from("Card Test")
   }

   fn update(&mut self, message: Self::Message) {
      match message {
         Self::Message::OnCardPressed => ()
      }
   }

   fn view(&mut self) -> Element<Self::Message> {
      let header = Text::new("Header");
      let body = Text::new("Body");
      let footer = Text::new("Footer");
      
      let card = Card::new(&mut self.card_state, header, body, footer)
         .min_width(127)
         .min_height(127);

      Container::new(card)
         .width(Length::Fill)
         .height(Length::Fill)
         .center_x()
         .center_y().into()
   }
}
