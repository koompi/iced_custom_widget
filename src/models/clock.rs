// use chrono::DateTime;
// use iced::{
//    executor, Application, Color, Command, Container, Element, Length, Point, Settings, Subscription, Vector, 
//    canvas::{self, Canvas, Stroke, Path, LineCap, layer, Drawable},
// };

// pub struct Clock {
//    pub now: DateTime<chrono::Local>,
//    pub clock: layer::Cache
// }

// #[derive(Debug, Clone, Copy)]
// enum Message {
//    Tick(DateTime<chrono::Local>)
// }

// impl Application for Clock {
//    type Executor = executor::Default;
//    type Message = Message;
//    type Flags = ();

//    fn new(_flags: ()) -> (Self, Command<Message>) {
//       (
//          Self {
//             now: chrono::Local::now(),
//          },
//          Command::none()
//       )
//    }

//    fn title(&self) -> String {
//       String::from("Clock - Iced")
//    }

//    fn update(&mut self, message: Message) -> Command<Message> {
//       match message {
//          Message::Tick(local_time) => {
//             let now = local_time;

//             if now != self.now {
//                self.now = now;
//                self.
//             }
//          }
//       }

//       Command::none()
//    }
// }