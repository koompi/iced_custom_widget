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
         Self::Message::OnCardPressed => {
            println!("Card Pressed");
         }
      }
   }

   fn view(&mut self) -> Element<Self::Message> {
      let header = Text::new("Header ggdfgdfg");
      let body = Text::new("Body fdgdfgdf fgdfsgsdgfdf");
      let footer = Text::new("Footer sdfdfgfdgdfgfd");
      
      let card = Card::new(&mut self.card_state).header(header).body(body).footer(footer)
         .spacing(20)
         .padding(10)
         .on_pressed(Self::Message::OnCardPressed);

      Container::new(card)
         .width(Length::Fill)
         .height(Length::Fill)
         .center_x()
         .center_y().into()
   }
}
