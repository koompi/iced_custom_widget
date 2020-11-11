use crate::components::stack::{Stack, Overflow};
use crate::components::outline_button::{self, OutlineButton};
use iced::{Container, Element, Length, Point, Sandbox, Settings, Text, Column, Checkbox};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleOverflow(bool),
    ButtonPressed
}

#[derive(Default)]
pub struct StackDemo {
    pub is_overflow: bool,
    pub btn_state: outline_button::State
}

impl StackDemo {
    pub fn init() -> iced::Result {
        Self::run(Settings {
            antialiasing: false,
            ..Settings::default()
        })
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
            Message::ButtonPressed => println!("button outline pressed")
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let overflow_checkbox =
            Checkbox::new(self.is_overflow, "Overflow", Message::ToggleOverflow);
        // let container1 = Container::new(Text::new("label1"))
        //     .width(Length::Units(200))
        //     .height(Length::Units(100));
        // let container2 = Container::new(Text::new("label2"))
        //     .width(Length::Units(150))
        //     .height(Length::Units(150));
        let button = OutlineButton::new(&mut self.btn_state, Text::new("Click me!"))
            .on_press(Message::ButtonPressed);
        let text1 = Text::new("label1");
        let text2 = Text::new("23223");
        let move_pos = Point::new(10., 10.);
        let stack_content = Stack::new()
            .push(text1, None)
            .push(text2, Some(move_pos));
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(stack_content)
            .push(overflow_checkbox)
            .push(button).into()
    }
}
