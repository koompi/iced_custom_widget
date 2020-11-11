use crate::components::icon::Icon;
// use crate::components::k_button::{self, KButtton};
// use crate::styles::custom_button::CustomButton;
use iced::{button, Align, Button, Column, Container, Element, Length, Text};

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
            AppMessage::AppClicked => {}
        }
    }

    pub fn view(&mut self) -> Element<AppMessage> {
        let icon = Icon::new(self.icon).size(127);
        icon.into()
        // let name = Text::new(&self.name).size(15);
        // let app = Column::new()
        //     .align_items(Align::Center)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .push(icon)
        //     .push(name);
        // let container = Container::new(app);
        // let app_btn = Button::new(&mut self.clickable, container)
        //     .padding(10)
        //     .on_press(AppMessage::AppClicked);
        // Container::new(app_btn)
        //     .center_x()
        //     .center_y()
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .into()
    }
}
