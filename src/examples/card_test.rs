use crate::components::card::{self, Card};
use iced::{
    pick_list, Align, Column, Container, Element, Length, PickList, Sandbox, Settings, Text,
};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Language {
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

impl Default for Language {
    fn default() -> Self {
        Self::Rust
    }
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
pub struct CardTest {
    pub card_state: card::State,
    pub pick_list: pick_list::State<Language>,
    pub selected_language: Language,
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnCardPressed,
    LanguageChanged(Language),
}

impl CardTest {
    pub fn init() -> iced::Result {
        CardTest::run(Settings::default())
    }
}

impl Sandbox for CardTest {
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
        // let body = Text::new("Body section");
        let body = Column::new()
            .align_items(Align::Center)
            .spacing(10)
            .push(Text::new("Which is your favorite language?"))
            .push(pick_list);
        let footer = Text::new("Footer section");

        let card = Card::new(&mut self.card_state)
            .header(header)
            .body(body)
            .footer(footer)
            .spacing(20)
            .padding(10)
            .on_pressed(Self::Message::OnCardPressed);

        Container::new(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
