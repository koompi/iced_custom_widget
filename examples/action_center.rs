#![allow(dead_code)]
use iced::{
    button, executor, slider, window, Align, Application, Button, Color, Column, Command,
    Container, Element, Length, ProgressBar, Row, Settings, Slider, Space, Subscription, Text,
};
use iced_custom_widget as icw;
use iced_native::window::Event;
use iced_native::Event::Window;
use icw::components::Icon;
use icw::styles::{buttons::ButtonStyle, containers::ContainerStyle, progressbar::SliderStyle};
use std::ops::Index;

use std::collections::HashMap;
#[derive(Default, Debug, Clone)]
pub struct ControlCenter<'a> {
    value: f32,
    progress_bar_slider: slider::State,
    value_bright: f32,
    progressbar_bright: slider::State,
    wifi: button::State,
    bluetooth: button::State,
    airplan: button::State,
    hostplot: button::State,
    night_mode: button::State,
    link_mode: button::State,
    lock_screen: button::State,
    full_screen: button::State,
    play_left: button::State,
    paly_right: button::State,
    paly_middle: button::State,
    map_con: button::State,
    settings_con: button::State,
    sound_con: button::State,
    music: Music<'a>,
    window_size: (u32, u32),
    vec_singers_songs: HashMap<&'a str, &'a str>,
    is_wifi: bool,
    is_bluetooth: bool,
    is_airplane: bool,
    is_hotspot: bool,
    is_nightmod: bool,
    is_link: bool,
    is_lock: bool,
    is_full: bool,
    is_play: bool,
    is_map: bool,
    is_settings: bool,
    is_sound: bool,
}
impl<'a> Index<&str> for ControlCenter<'a> {
    type Output = &'a str;
    fn index(&self, key: &str) -> &&'a str {
        &self.vec_singers_songs.index(key)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Music<'a> {
    m_singer: &'a str,
    m_song: &'a str,
    m_published: &'a str,
    m_index: u32,
}

impl<'a> Music<'a> {
    fn new(name: &'a str, song: &'a str, published: &'a str) -> Self {
        Self {
            m_singer: name,
            m_song: song,
            m_published: published,
            m_index: 0,
        }
    }
    fn get_singer(&self) -> &str {
        self.m_singer
    }
    fn get_song(&self) -> &str {
        self.m_song
    }
    fn get_published(&self) -> &str {
        self.m_published
    }
}
#[derive(Debug, Clone)]
pub enum ControlCenterMsg {
    Escape,
    CloseApp,
    WindowResize((u32, u32)),
    SliderChanged(f32),
    BrightChanged(f32),
    WifiSwitch,
    BluetoothSwitch,
    AirplaneSwitch,
    HotspotSwitch,
    NightModeSwitch,
    LinkSwitch,
    LockSwitch,
    FullScreenSwitch,
    PlayLeft,
    PlayRight,
    PlayMiddle,
    MapSwitch,
    SettingsSwitch,
    SoundSwitch,
}

impl<'a> Application for ControlCenter<'a> {
    type Executor = executor::Default;
    type Message = ControlCenterMsg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<ControlCenterMsg>) {
        let mut vec_singer: Vec<&str> = vec!["Sin Sisamuth ", "Taylor Swift", "Charlie Puth"];
        let mut vec_song: Vec<&str> = vec!["Champa Batdombong", "You belong with me", "Attention"];
        let mut singers_songs: HashMap<&str, &str> = HashMap::new();
        for (i, j) in vec_singer.iter_mut().zip(vec_song.iter_mut()) {
            singers_songs.insert(i, j);
        }
        (
            Self {
                vec_singers_songs: singers_songs,
                music: Music::new("Charlie Puth", "See you Again", "March 15 2015"),
                ..ControlCenter::default()
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("ActionCenter")
    }
    fn update(&mut self, message: ControlCenterMsg) -> Command<ControlCenterMsg> {
        match message {
            ControlCenterMsg::SliderChanged(x) => self.value = x,
            ControlCenterMsg::WifiSwitch => {
                self.is_wifi = !self.is_wifi;
            }
            ControlCenterMsg::BluetoothSwitch => {
                self.is_bluetooth = !self.is_bluetooth;
            }
            ControlCenterMsg::HotspotSwitch => {
                self.is_hotspot = !self.is_hotspot;
            }
            ControlCenterMsg::AirplaneSwitch => {
                self.is_airplane = !self.is_airplane;
            }
            ControlCenterMsg::NightModeSwitch => {
                self.is_nightmod = !self.is_nightmod;
            }
            ControlCenterMsg::LockSwitch => {
                self.is_lock = !self.is_lock;
            }
            ControlCenterMsg::LinkSwitch => {
                self.is_link = !self.is_link;
            }
            ControlCenterMsg::FullScreenSwitch => {
                self.is_full = !self.is_full;
            }
            ControlCenterMsg::BrightChanged(val) => {
                self.value_bright = val;
            }
            ControlCenterMsg::MapSwitch => {
                self.is_map = !self.is_map;
            }
            ControlCenterMsg::SettingsSwitch => {
                self.is_settings = !self.is_settings;
            }
            ControlCenterMsg::SoundSwitch => {
                self.is_sound = !self.is_sound;
            }
            ControlCenterMsg::PlayLeft => {
                if self.music.m_index as usize == 0 {
                    self.music.m_index = (self.vec_singers_songs.len() - 1) as u32;
                    println!("condition met");
                } else {
                    self.music.m_index -= 1;
                }
                println!("index: {}", self.music.m_index);
                // let index = self.vec_singers_songs.index("Charlie Puth");
                let index = self.music.m_index;
                let mut singer = "";
                self.vec_singers_songs
                    .keys()
                    .enumerate()
                    .for_each(|(idx, val)| {
                        if index as usize == idx {
                            singer = val;
                            println!("singer name: {}", singer);
                        }
                    });
                self.music.m_singer = singer;
                let song = self.vec_singers_songs.index(singer);
                self.music.m_song = song;
            }
            ControlCenterMsg::WindowResize((w, h)) => {
                self.window_size.0 = w;
                self.window_size.1 = h;
            }
            ControlCenterMsg::PlayMiddle => {
                self.is_play = !self.is_play;
            }
            ControlCenterMsg::PlayRight => {
                if self.music.m_index as usize == self.vec_singers_songs.len() - 1 {
                    self.music.m_index = 0;
                    println!("condition met");
                } else {
                    self.music.m_index += 1;
                }
                println!("index: {}", self.music.m_index);
                // let index = self.vec_singers_songs.index("Charlie Puth");
                let index = self.music.m_index;
                let mut singer = "";
                self.vec_singers_songs
                    .keys()
                    .enumerate()
                    .for_each(|(idx, val)| {
                        if index as usize == idx {
                            singer = val;
                            println!("singer name: {}", singer);
                        }
                    });
                self.music.m_singer = singer;
                let song = self.vec_singers_songs.index(singer);
                self.music.m_song = song;
            }
            _ => {}
        }
        Command::none()
    }
    fn subscription(&self) -> Subscription<ControlCenterMsg> {
        iced_native::subscription::events_with(|event, status| {
            if let iced_native::event::Status::Captured = status {
                return None;
            }

            match event {
                Window(Event::Resized { width, height }) => {
                    println!("width: {}", width);
                    Some(ControlCenterMsg::WindowResize((width, height)))
                }
                iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => match key_code {
                    iced::keyboard::KeyCode::W => {
                        if modifiers.control {
                            Some(ControlCenterMsg::CloseApp)
                        } else {
                            None
                        }
                    }
                    iced::keyboard::KeyCode::Escape => Some(ControlCenterMsg::Escape),
                    _ => None,
                },
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<ControlCenterMsg> {
        let wifi = control_view(
            "Wifi",
            &mut self.wifi,
            self.is_wifi,
            '\u{f1eb}',
            ControlCenterMsg::WifiSwitch,
        );
        let bluetooth = control_view(
            "Bluetooth",
            &mut self.bluetooth,
            self.is_bluetooth,
            '\u{f03d}',
            ControlCenterMsg::BluetoothSwitch,
        );
        let airplane = control_view(
            "",
            &mut self.airplan,
            self.is_airplane,
            '\u{f072}',
            ControlCenterMsg::AirplaneSwitch,
        );
        let hotspot = control_view(
            "",
            &mut self.hostplot,
            self.is_hotspot,
            '\u{f0c1}',
            ControlCenterMsg::HotspotSwitch,
        );
        let locker = control_view(
            "",
            &mut self.lock_screen,
            self.is_lock,
            '\u{f023}',
            ControlCenterMsg::LockSwitch,
        );
        let link = control_view(
            "",
            &mut self.link_mode,
            self.is_link,
            '\u{f0c1}',
            ControlCenterMsg::LinkSwitch,
        );
        let full_screen = control_view(
            "",
            &mut self.full_screen,
            self.is_full,
            '\u{f066}',
            ControlCenterMsg::FullScreenSwitch,
        );
        let night_mod = control_view(
            "",
            &mut self.night_mode,
            self.is_nightmod,
            '\u{f186}',
            ControlCenterMsg::NightModeSwitch,
        );
        let right_content = Column::new()
            .spacing(4)
            .width(Length::FillPortion(1))
            .push(
                Container::new(
                    Column::new()
                        .width(Length::Fill)
                        .align_items(Align::Center)
                        .padding(10)
                        .spacing(10)
                        .push(
                            Container::new(Icon::new('\u{f001}'))
                                .width(Length::Shrink)
                                .style(ContainerStyle::LightGrayCircle)
                                .height(Length::Shrink)
                                .padding(10),
                        )
                        .push(
                            Column::new()
                                .align_items(Align::Center)
                                .spacing(10)
                                .push(Text::new(self.music.m_song).size(18))
                                .push(Text::new(self.music.m_singer).size(16))
                                .push(Text::new(self.music.m_published).size(12)),
                        )
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .spacing(8)
                                .push(
                                    Button::new(&mut self.play_left, Icon::new('\u{f104}'))
                                        .style(ButtonStyle::Transparent)
                                        .on_press(ControlCenterMsg::PlayLeft),
                                )
                                .push(
                                    Button::new(
                                        &mut self.paly_middle,
                                        Icon::new({
                                            if self.is_play {
                                                '\u{f04c}'
                                            } else {
                                                '\u{f04b}'
                                            }
                                        }),
                                    )
                                    .on_press(ControlCenterMsg::PlayMiddle)
                                    .style(ButtonStyle::Transparent),
                                )
                                .push(
                                    Button::new(&mut self.paly_right, Icon::new('\u{f105}'))
                                        .style(ButtonStyle::Transparent)
                                        .on_press(ControlCenterMsg::PlayRight),
                                ),
                        ),
                )
                .width(Length::Fill)
                .height(Length::Shrink)
                .style(ContainerStyle::LightGrayCircle)
                .padding(10),
            )
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .spacing(4)
                                .push(Icon::new('\u{f185}').size(32))
                                .push(Text::new("Bright")),
                        )
                        .push(
                            ProgressBar::new(0.0..=100.0, self.value_bright)
                                .height(Length::from(30))
                                .style(SliderStyle::WhiteGrayCircle(255, 255, 255, 1.0, 16.0)),
                        )
                        .push(
                            Slider::new(
                                &mut self.progressbar_bright,
                                0.0..=100.0,
                                self.value_bright,
                                ControlCenterMsg::BrightChanged,
                            )
                            .step(0.01),
                        ),
                )
                .padding(10)
                .width(Length::Fill)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Row::new()
                    .spacing(4)
                    .push(control_view(
                        "",
                        &mut self.map_con,
                        self.is_map,
                        '\u{f041}',
                        ControlCenterMsg::MapSwitch,
                    ))
                    .push(control_view(
                        "",
                        &mut self.settings_con,
                        self.is_settings,
                        '\u{f013}',
                        ControlCenterMsg::SettingsSwitch,
                    ))
                    .push(control_view(
                        "",
                        &mut self.sound_con,
                        self.is_sound,
                        '\u{f028}',
                        ControlCenterMsg::SoundSwitch,
                    )),
            );
        let left_content = Column::new()
            .width(Length::FillPortion(1))
            .spacing(4)
            .push(wifi)
            .push(bluetooth)
            .push(
                Row::new()
                    .spacing(4)
                    .push(airplane)
                    .push(hotspot)
                    .push(night_mod),
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .spacing(4)
                                .push(Icon::new('\u{f025}').size(32))
                                .push(Text::new("Headphone")),
                        )
                        .push(
                            ProgressBar::new(0.0..=100.0, self.value)
                                .height(Length::from(30))
                                .style(SliderStyle::WhiteGrayCircle(255, 255, 255, 1.0, 16.0)),
                        )
                        .push(
                            Slider::new(
                                &mut self.progress_bar_slider,
                                0.0..=100.0,
                                self.value,
                                ControlCenterMsg::SliderChanged,
                            )
                            .step(0.01),
                        ),
                )
                .style(ContainerStyle::LightGrayCircle)
                .padding(10),
            )
            .push(
                Row::new()
                    .align_items(Align::Center)
                    .spacing(4)
                    .push(link)
                    .push(locker)
                    .push(full_screen),
            );
        Container::new(if self.window_size.0 < 580 {
            Container::new(
                Column::new()
                    .padding(10)
                    .push(left_content)
                    .push(right_content)
                    .spacing(10),
            )
        } else {
            Container::new(
                Row::new()
                    .padding(10)
                    .push(left_content)
                    .push(right_content)
                    .spacing(10),
            )
        })
        .style(ContainerStyle::White)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

// helper functions
fn control_view<'a>(
    name: &str,
    state: &'a mut button::State,
    status: bool,
    icon: char,
    f: ControlCenterMsg,
) -> Button<'a, ControlCenterMsg> {
    Button::new(
        state,
        Row::new()
            .padding(10)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(Space::with_width(Length::Fill))
            .push(Icon::new(icon).size(32))
            .push(Text::new(name))
            .push(Space::with_width(Length::Fill)),
    )
    .width(Length::Fill)
    .on_press(f)
    .style(if status {
        ButtonStyle::CircleRadius(7, 92, 182, 1.0, 10.0, Color::WHITE)
    } else {
        ButtonStyle::CircleRadius(215, 219, 221, 0.5, 10.0, Color::BLACK)
    })
}

fn init() -> iced::Result {
    ControlCenter::run(Settings {
        window: window::Settings {
            size: (400, 400),
            min_size: Some((300, 800)),
            max_size: Some((600, 1080)),
            resizable: true,
            decorations: true,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
fn main() {
    match init() {
        Ok(()) => println!("run sucessfully"),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Hello World from Koompi Bluetooth");
}
