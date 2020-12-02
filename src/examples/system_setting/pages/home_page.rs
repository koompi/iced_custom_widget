use iced::{
   Container, Space, Length,
};
use super::super::pages::PagesMessage;

pub fn home_page<'a>() -> Container<'a, PagesMessage> {
   Container::new(Space::with_width(Length::Shrink))
}