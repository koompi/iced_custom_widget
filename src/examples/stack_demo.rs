use crate::components::outline_button::{self, OutlineButton};
use crate::components::stack::{Overflow, Stack};
use crate::styles::custom_styles;
use crate::utils::themes::Theme;
use iced::{Checkbox, Column, Container, Element, Length, Point, Sandbox, Settings, Text};

#[derive(Debug, Clone)]
pub enum Message {
   ToggleOverflow(bool),
   ButtonPressed,
}

#[derive(Default)]
pub struct StackDemo {
   pub is_overflow: bool,
   pub btn_state: outline_button::State,
}

impl StackDemo {
   pub fn init() {
      Self::run(Settings {
         antialiasing: false,
         ..Settings::default()
      })
      .unwrap();
   }
}

impl Sandbox for StackDemo {
   type Message = Message;

   fn new() -> Self {
      Self::default()
   }

   fn title(&self) -> String {
      String::from("Stack Widget Demo")
   }

   fn update(&mut self, message: Self::Message) {
      match message {
         Message::ToggleOverflow(is_overflow) => self.is_overflow = is_overflow,
         Message::ButtonPressed => println!("button outline pressed"),
      }
   }

   fn view(&mut self) -> Element<Self::Message> {
      let overflow_checkbox = Checkbox::new(self.is_overflow, "Overflow", Message::ToggleOverflow)
         .style(custom_styles::CustomCheckbox::Default(Theme::light().palette));
      let container1 = Container::new(Text::new("label1"))
         .width(Length::Units(200))
         .height(Length::Units(100))
         .style(custom_styles::CustomContainer::BrightBackground(Theme::light().palette));
      let container2 = Container::new(Text::new("label2"))
         .width(Length::Units(150))
         .height(Length::Units(150))
         .style(custom_styles::CustomContainer::BrightForeground(Theme::light().palette));
      let button = OutlineButton::new(&mut self.btn_state, Text::new("Click me!")).on_press(Message::ButtonPressed);
      let move_pos = Point::new(20., 30.);
      let stack_content = Stack::new()
         .overflow(Overflow::Clip)
         .push(container1, None)
         .push(container2, Some(move_pos));
      Column::new()
         .width(Length::Fill)
         .height(Length::Fill)
         .push(stack_content)
         .push(overflow_checkbox)
         .push(button)
         .into()
   }
}
