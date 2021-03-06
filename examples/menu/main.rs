mod app;
use app::{App, AppMessage};
use cw::components::{Grid, IconBrands};
use cw::styles::custom_styles::CustomTextInput;
use cw::utils::Theme;
use iced::{
    executor, scrollable, text_input, window, Align, Application, Color, Column, Command,
    Container, Element, Length, Row, Scrollable, Settings, TextInput,
};
use iced_custom_widget as cw;
pub struct Menu {
    input_search: text_input::State,
    search_text: String,
    applications: Vec<App>,
    filtered_application: Vec<App>,
    scroll: scrollable::State,
}

impl Menu {
    pub fn init() -> iced::Result {
        Menu::run(Settings {
            default_text_size: 13,
            window: window::Settings {
                always_on_top: true,
                transparent: true,
                ..window::Settings::default()
            },
            ..Settings::default()
        })
    }
}

#[derive(Debug, Clone)]
pub enum MenuMessage {
    SearchChanged(String),
    ActionSearch,
    AppMessage(usize, AppMessage),
}

impl Application for Menu {
    type Executor = executor::Default;
    type Message = MenuMessage;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let applications = vec![
            App::new(IconBrands::Airbnb, "Airbnb".to_string()),
            App::new(IconBrands::AppStore, "App Store".to_string()),
            App::new(IconBrands::AmazonWebServicesAws, "AWS".to_string()),
            App::new(IconBrands::Chrome, "Google Chrom".to_string()),
            App::new(IconBrands::Codepen, "CodePen".to_string()),
            App::new(IconBrands::DigitalOcean, "Digital Ocean".to_string()),
            App::new(IconBrands::Discord, "Discord".to_string()),
            App::new(IconBrands::Docker, "Docker".to_string()),
            App::new(IconBrands::Facebook, "Facebook".to_string()),
            App::new(IconBrands::FacebookMessenger, "Messenger".to_string()),
            App::new(IconBrands::Firefox, "Firefox".to_string()),
            App::new(IconBrands::Github, "Github".to_string()),
            App::new(IconBrands::GoogleDrive, "Google Drive".to_string()),
            App::new(IconBrands::GooglePlay, "Play Store".to_string()),
            App::new(IconBrands::Instagram, "Instagram".to_string()),
            App::new(IconBrands::ItunesNote, "Music".to_string()),
            App::new(IconBrands::Linkedin, "Linkedin".to_string()),
            App::new(IconBrands::Safari, "Safari".to_string()),
            App::new(IconBrands::Telegram, "Telegram".to_string()),
            App::new(IconBrands::StackOverflow, "StackOverflow".to_string()),
            App::new(IconBrands::Trello, "Trello".to_string()),
            App::new(IconBrands::Twitter, "Twitter".to_string()),
            App::new(IconBrands::Uber, "Uber".to_string()),
            App::new(IconBrands::Uikit, "UIKit".to_string()),
            App::new(IconBrands::Vk, "VK".to_string()),
            App::new(IconBrands::WhatSApp, "Whatsapp".to_string()),
            App::new(IconBrands::YahooLogo, "Yahoo".to_string()),
        ];
        (
            Self {
                input_search: text_input::State::new(),
                search_text: String::new(),
                applications: applications.clone(),
                filtered_application: applications.clone(),
                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Menu")
    }

    fn update(&mut self, message: MenuMessage) -> Command<Self::Message> {
        match message {
            MenuMessage::SearchChanged(text) => {
                self.search_text = text;
                self.filtered_application = self
                    .applications
                    .iter()
                    .filter(|app| {
                        app.name
                            .to_lowercase()
                            .contains(&self.search_text.to_lowercase())
                    })
                    .cloned()
                    .collect();
            }
            MenuMessage::ActionSearch => {
                self.filtered_application = self
                    .applications
                    .iter()
                    .filter(|app| {
                        app.name
                            .to_lowercase()
                            .contains(&self.search_text.to_lowercase())
                    })
                    .cloned()
                    .collect();
            }
            MenuMessage::AppMessage(i, app_message) => {
                if let Some(app) = self.applications.get_mut(i) {
                    app.update(app_message);
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let search = TextInput::new(
            &mut self.input_search,
            "Search",
            &mut self.search_text,
            MenuMessage::SearchChanged,
        )
        .padding(10)
        .max_width(800)
        .width(Length::Units(500))
        .size(17)
        .style(CustomTextInput::Default(Theme::light().palette))
        .on_submit(MenuMessage::ActionSearch);
        let search_section = Container::new(search)
            .center_x()
            .center_y()
            .width(Length::Fill);
        let search_bar = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(search_section);

        let menu: Element<_> = self
            .filtered_application
            .iter_mut()
            .enumerate()
            .fold(Grid::new().column_width(175), |grid, (i, app)| {
                grid.push(
                    app.view()
                        .map(move |message| MenuMessage::AppMessage(i, message)),
                )
            })
            .into();

        let content = Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(search_bar)
            .push(menu);

        Scrollable::new(&mut self.scroll)
            .padding(30)
            .push(
                Container::new(content)
                    .width(Length::Fill)
                    .center_y()
                    .center_x(),
            )
            .into()
    }

    fn mode(&self) -> window::Mode {
        window::Mode::Fullscreen
    }

    fn background_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.3)
    }

    // fn scale_factor(&self) -> f64 {
    //    self.scale_state.scale as f64
    // }
}
#[allow(unused_must_use)]
fn main() {
    Menu::init();
}
