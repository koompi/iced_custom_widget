use iced_custom_widget as cw;
use cw::components::number_input::{self, NumberInput};
use cw::styles::custom_card::CustomCard;
use iced::{pick_list, Column, Container, Element, Length, PickList, Sandbox, Settings, Text};
use smart_default::SmartDefault;
use std::fmt::{Display, Formatter, Result};

#[derive(Default)]
pub struct NumberInputDemo {
   state: number_input::State,
   value: u8
}

impl NumberInputDemo {
   pub fn init() -> iced::Result {
      std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.2");
      NumberInputDemo::run(Settings {
         // default_text_size: 13,
         ..Settings::default()
      })
   }
}

#[derive(Debug, Clone)]
pub enum NumInpMessage {
   NumInpChanged(u8),
}

impl Sandbox for NumberInputDemo {
   type Message = NumInpMessage;

   fn new() -> Self {
      Self::default()
   }

   fn title(&self) -> String {
      String::from("Number Input Demo")
   }

   fn update(&mut self, message: Self::Message) {
      match message {
         Self::Message::NumInpChanged(val) => {
            self.value = val;
         }
      }
   }

   fn view(&mut self) -> Element<Self::Message> {
      let num_inp = NumberInput::new(
         &mut self.state,
         self.value,
         50,
         Self::Message::NumInpChanged,
      );
      Container::new(num_inp)
         .width(Length::Fill)
         .height(Length::Fill)
         .center_x()
         .center_y()
         .into()
   }
}
#[allow(unused_must_use)]
fn main() {
   NumberInputDemo::init(); 
}
