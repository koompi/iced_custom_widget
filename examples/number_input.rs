use iced_custom_widget as cw;
use cw::components::number_input::{self, NumberInput};
use iced::{Container, Element, Length, Sandbox, Settings, Text, Row, Align, window};

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
         window: window::Settings {
            size: (250, 200),
            ..Default::default()
         },
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
         255.0,
         Self::Message::NumInpChanged,
      ).step(1.5);
      Container::new(
         Row::new().spacing(10).align_items(Align::Center)
         .push(lb_minute)
         .push(txt_minute)
      ).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
   }
}
#[allow(unused_must_use)]
fn main() {
   NumberInputDemo::init(); 
}

mod style {
   use iced_custom_widget::styles::number_input::{Style, StyleSheet};
   use iced::{Color, Background};
   pub struct CustomNumInput;

   impl StyleSheet for CustomNumInput {
      fn active(&self) -> Style { 
         Style {
            button_background: Some(Background::Color(Color::from_rgb8(15, 85, 179))),
            icon_color: Color::WHITE,
            ..Style::default()
         }
      }
   }
}