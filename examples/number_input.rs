use iced_custom_widget as cw;
use cw::components::number_input::{self, NumberInput};
use iced::{Container, Element, Length, Sandbox, Settings, Text, Row, Align};

#[derive(Default)]
pub struct NumberInputDemo {
   state: number_input::State,
   value: f32
}

impl NumberInputDemo {
   pub fn init() -> iced::Result {
      std::env::set_var("WINIT_X11_SCALE_FACTOR", "1.2");
      NumberInputDemo::run(Settings {
         default_text_size: 14,
         ..Settings::default()
      })
   }
}

#[derive(Debug, Clone)]
pub enum NumInpMessage {
   NumInpChanged(f32),
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
      let lb_minute = Text::new("Minutes:");
      let txt_minute = NumberInput::new(
         &mut self.state,
         self.value,
         27.5,
         Self::Message::NumInpChanged,
      ).padding(5).step(1.5);
      Container::new(
         Row::new().spacing(10).align_items(Align::Center)
         .push(lb_minute)
         .push(txt_minute)
      ).width(Length::Fill).height(Length::Fill).into()
   }
}
#[allow(unused_must_use)]
fn main() {
   NumberInputDemo::init(); 
}
