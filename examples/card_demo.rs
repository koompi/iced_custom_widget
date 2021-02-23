use iced_custom_widget as cw;
use cw::card::{self, Card};
use cw::stepper::{self, Stepper};
use iced::{pick_list, Column, Container, Element, Length, PickList, Sandbox, Settings, Text};
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq, SmartDefault)]
pub enum Language {
   #[default]
   Rust,
   Elm,
   Ruby,
   Haskell,
   C,
   Javascript,
   Other,
}

impl Language {
   pub const ALL: [Language; 7] = [
      Language::C,
      Language::Elm,
      Language::Haskell,
      Language::Javascript,
      Language::Other,
      Language::Ruby,
      Language::Rust,
   ];
}

impl Display for Language {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(
         f,
         "{}",
         match self {
            Language::C => "C",
            Language::Elm => "Elm",
            Language::Haskell => "Haskell",
            Language::Javascript => "JavaScript",
            Language::Other => "Other",
            Language::Ruby => "Ruby",
            Language::Rust => "Rust",
         }
      )
   }
}

#[derive(Default)]
pub struct CardDemo {
   card_state: card::State,
   pick_list: pick_list::State<Language>,
   selected_language: Language,
   scale_state: ScaleState,
}

#[derive(SmartDefault)]
pub struct ScaleState {
   scale: f32,
   stepper_state: stepper::State,
}
impl CardDemo {
   pub fn init() -> iced::Result {
      std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.2");
      CardDemo::run(Settings {
         // default_text_size: 13,
         ..Settings::default()
      })
   }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
   OnCardPressed,
   LanguageChanged(Language),
   ScaleChanged(f32),
}

impl Sandbox for CardDemo {
   type Message = CardMessage;

   fn new() -> Self {
      Self::default()
   }

   fn title(&self) -> String {
      String::from("Card with picklist")
   }

   fn update(&mut self, message: Self::Message) {
      match message {
         Self::Message::OnCardPressed => {
            println!("Card Pressed");
         }
         Self::Message::LanguageChanged(language) => {
            self.selected_language = language;
         }
         Self::Message::ScaleChanged(scale) => {
            self.scale_state.scale = scale;
         }
      }
   }

   fn view(&mut self) -> Element<Self::Message> {
      let pick_list = PickList::new(
         &mut self.pick_list,
         &Language::ALL[..],
         Some(self.selected_language),
         Self::Message::LanguageChanged,
      );
      let header = Text::new("Header section");
      let body = Text::new("Which is your favorite language?");
      let footer = Text::new("Footer section");
      let card = Card::new(&mut self.card_state)
         .header(header)
         .body(body)
         .footer(footer)
         .spacing(20)
         .padding(10)
         .on_pressed(Self::Message::OnCardPressed);
      let resizer = Stepper::new(
         &mut self.scale_state.stepper_state,
         self.scale_state.scale,
         50.50,
         Self::Message::ScaleChanged,
      ).step(2.5);
      let col = Column::new().push(card).push(resizer).push(pick_list);
      Container::new(col)
         .width(Length::Fill)
         .height(Length::Fill)
         .center_x()
         .center_y()
         .into()
   }
}
#[allow(unused_must_use)]
fn main() {
   CardDemo::init(); 
}