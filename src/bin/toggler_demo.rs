fn main() {
    init();
}
use iced_custom_widget as icw;
use icw::components::Toggler;
use iced::{
    executor, window, Align, Application, Checkbox, Color, Column, Command, Container, Element,
    Length, Settings, Subscription, Text,
};
#[derive(Default, Debug)]
pub struct Event {
    last: Vec<iced_native::Event>,
    enable: bool,
    value_toggled: bool,
}
pub fn init() {
    match Event::run(Settings::default()) {
        Ok(exit_code) => println!("exit code: {:?}", exit_code),
        Err(e) => eprintln!("Error: {:?}", e)
    }
}
#[derive(Debug, Clone)]
pub enum EventMessage {
    EventOccured(iced_native::Event),
    Toogled(bool),
    TogglerChanged(bool),
    CloseApp,
    Escape
}
impl Application for Event {
    type Executor = executor::Default;
    type Message = EventMessage;
    type Flags = ();
    fn new(_flags: ()) -> (Event, Command<EventMessage>) {
        (Event::default(), Command::none())
    }
    fn title(&self) -> String {
        String::from("Event Handler")
    }
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            EventMessage::EventOccured(event) => {
                self.last.push(event);
                if self.last.len() > 5 {
                    self.last.remove(0);
                }
            }
            EventMessage::Toogled(is_toggled) => {
                self.enable = is_toggled;
            }
            EventMessage::TogglerChanged(is_toggled) => {
                self.value_toggled = is_toggled;
                println!("You toggled the message: {}", is_toggled);
            }
            EventMessage::CloseApp  => {
                println!("Application close");
            }
            EventMessage::Escape => {
                println!("Escape key pressed.")
            }
        }

        Command::none()
    }
    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events_with(|event, status| {
            if let iced_native::event::Status::Captured = status {
                return None;
            }

            match event {
                iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => match key_code {
                    iced::keyboard::KeyCode::W => {
                        if modifiers.control {
                            Some(EventMessage::CloseApp)
                        } else {
                            None
                        }
                    }
                    iced::keyboard::KeyCode::Escape => Some(EventMessage::Escape),
                    _ => None,
                },
                _ => None,
            }
        })
    }
    fn view(&mut self) -> Element<Self::Message> {
        let event = self
            .last
            .iter()
            .fold(Column::new().spacing(10), |column, event| {
                column.push(Text::new(format!("{:?}", event)).size(40))
            });
        let toggle = Checkbox::new(
            self.enable,
            "Toggle to enter the event",
            EventMessage::Toogled,
        );
        let col = Column::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(event)
            .push(
                Toggler::new(
                    self.value_toggled,
                    String::from("Enable Network"),
                    EventMessage::TogglerChanged,
                )
                .width(Length::Shrink),
            )
            .push(Toggler::new(
                self.value_toggled,
                String::from("Manual"),
                EventMessage::TogglerChanged,
            ))
            .push(toggle);

        Container::new(col)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn mode(&self) -> window::Mode {
        window::Mode::Windowed
    }

    fn background_color(&self) -> Color {
        Color::from_rgba8(127, 143, 166, 1.0)
    }
    fn scale_factor(&self) -> f64 {
        1.0
    }
}


