use iced::{
    button, checkbox, executor, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, HorizontalAlignment, Length, Row, Rule, Scrollable,
    Settings, Space, Subscription, Text, TextInput, VerticalAlignment,
};
use iced_custom_widget as icw;
use icw::components::{Icon, Toggler};
use icw::styles::{
    buttons::ButtonStyle, containers::ContainerStyle, rules::RuleStyle, text_input::InputStyle,
};
#[derive(Default, Debug, Clone)]
pub struct KBleutooth {
    is_enable: bool,
    is_allowed: bool,
    is_shown: bool,
    is_shown_settings: bool,
    edit_dev: button::State,
    show_settings: button::State,
    refresh: button::State,
    dev_name: text_input::State,
    dev_name_val: String,
    bluetooth_settings: BluetoothSettings,
    btn_refresh: button::State,
    vector_bluetooths: Vec<(BluetoothDevType, String, BluetoothStatus)>,
    scroll_area: scrollable::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothStatus {
    Connected,
    Connecting,
    NoConnected,
    DisConnected,
}
#[derive(Debug, Clone)]
pub enum BluetoothDevType {
    SmartPhone,
    Computer,
    Headphone,
    Unknown,
}
impl Default for BluetoothDevType {
    fn default() -> Self {
        BluetoothDevType::SmartPhone
    }
}

#[derive(Debug, Clone)]
pub enum KBleutoothMsg {
    DevEdited,
    DevEditedVal(String),
    DevEnabled(bool),
    DevAllowed(bool),
    DevRefreshed,
    DevSettingsShown,
    DevShowNameless(bool),
    CloseApp,
    Escape,
    BluetoothSettingsMsg(BluetoothSettingsMsg),
}

impl Application for KBleutooth {
    type Executor = executor::Default;
    type Message = KBleutoothMsg;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<KBleutoothMsg>) {
        let simpler_code = |b_type: BluetoothDevType, b_ssid: &str, b_status: BluetoothStatus| {
            (b_type, b_ssid.to_string(), b_status)
        };
        let mut init_vec_state: Vec<(BluetoothDevType, String, BluetoothStatus)> = Vec::new();
        for _i in 1..=10 {
            init_vec_state.push(simpler_code(
                BluetoothDevType::Computer,
                "Mi Smart Band 5",
                BluetoothStatus::NoConnected,
            ));
        }
        (
            Self {
                vector_bluetooths: init_vec_state,
                bluetooth_settings: BluetoothSettings::new(),
                ..KBleutooth::default()
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Bluetooth")
    }
    fn update(&mut self, message: KBleutoothMsg) -> Command<KBleutoothMsg> {
        use KBleutoothMsg::*;
        match message {
            DevEnabled(is_enable) => {
                self.is_enable = is_enable;
                Command::none()
            }
            DevAllowed(is_allow) => {
                self.is_allowed = is_allow;
                Command::none()
            }
            DevShowNameless(data) => {
                self.is_shown = data;
                Command::none()
            }
            CloseApp => {
                println!("Applicaiton close:");
                Command::none()
            }
            BluetoothSettingsMsg(msg) => {
                self.bluetooth_settings.update(msg);
                Command::none()
            }
            DevSettingsShown => {
                self.is_shown_settings = !self.is_shown_settings;
                Command::none()
            }
            Escape => {
                println!("Escape key pressed: ");
                self.is_shown_settings = !self.is_shown_settings;
                Command::none()
            }
            _ => Command::none(),
        }
    }
    fn subscription(&self) -> Subscription<KBleutoothMsg> {
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
                            Some(KBleutoothMsg::CloseApp)
                        } else {
                            None
                        }
                    }
                    iced::keyboard::KeyCode::Escape => Some(KBleutoothMsg::Escape),
                    _ => None,
                },
                _ => None,
            }
        })
    }

    fn view(&mut self) -> Element<KBleutoothMsg> {
        let inner_layout = Container::new(
            Column::new()
                .spacing(10)
                .push(
                    Row::new()
                        .push(
                            Row::new()
                                .spacing(4)
                                .push(Text::new("sna-dell"))
                                .push(Icon::new('\u{f304}')),
                        )
                        .push(Toggler::new(
                            self.is_enable,
                            String::from(""),
                            KBleutoothMsg::DevEnabled,
                        )),
                )
                .push(Rule::horizontal(10).style(RuleStyle {}))
                .push(if self.is_enable {
                    Row::new()
                        .push(Text::new(
                            "Allow other Bluetooth devices to find this device",
                        ))
                        .push(Toggler::new(
                            self.is_allowed,
                            String::from(""),
                            KBleutoothMsg::DevAllowed,
                        ))
                } else {
                    Row::new().push(Text::new(
                        "Enable Bluetooth for devices (Mouse, Keyboard, Headphone)",
                    ))
                }),
        )
        .width(Length::Fill)
        .padding(10)
        .style(ContainerStyle::LightGrayCircle);
        let know_devices = Column::new()
            .spacing(10)
            .push(Text::new("My Devices").size(24))
            .push(
                Column::new()
                    .padding(10)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(6)
                            .push(Icon::new('\u{f10b}'))
                            .push(Text::new("Linux"))
                            .push(Space::with_width(Length::Fill))
                            .push(
                                Row::new().spacing(4).push(Text::new("Not Connected")).push(
                                    Button::new(&mut self.show_settings, Icon::new('\u{f105}'))
                                        .on_press(KBleutoothMsg::DevSettingsShown)
                                        .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                                ),
                            ),
                    ),
            );
        let other_devices = Column::new()
            .spacing(10)
            .push(Text::new("Other Devices").size(24))
            .push(
                Row::new()
                    .push(Checkbox::new(
                        self.is_shown,
                        "Show Bluetooth devices without names",
                        KBleutoothMsg::DevShowNameless,
                    ))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(&mut self.btn_refresh, Icon::new('\u{f021}'))
                            .on_press(KBleutoothMsg::DevRefreshed)
                            .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
                    ),
            )
            .push(self.vector_bluetooths.iter_mut().fold(
                Column::new().padding(10).spacing(16),
                |column, (b_type, b_ssid, b_status)| {
                    column.push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(4)
                            .push(
                                Icon::new(match b_type {
                                    BluetoothDevType::Computer => '\u{f108}',
                                    BluetoothDevType::Headphone => '\u{f3cd}',
                                    BluetoothDevType::SmartPhone => '\u{f58f}',
                                    BluetoothDevType::Unknown => '\u{f17c}',
                                })
                                .size(24),
                            )
                            .push(Text::new(b_ssid.as_str()))
                            .push(Space::with_width(Length::Fill))
                            .push(Text::new(match b_status {
                                BluetoothStatus::Connected => "Connected",
                                BluetoothStatus::Connecting => "Connecting",
                                BluetoothStatus::DisConnected => "Disconnected",
                                BluetoothStatus::NoConnected => "Not connected",
                            })),
                    )
                },
            ));
        let scroll_conent = Scrollable::new(&mut self.scroll_area)
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .push(
                Column::new()
                    .spacing(20)
                    .push(inner_layout)
                    .push(know_devices)
                    .push(other_devices),
            );
        let embbeded_layout = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                scroll_conent
                    .padding(10)
                    .scroller_width(4)
                    .scrollbar_width(4),
            )
            .push(if self.is_shown_settings {
                self.bluetooth_settings
                    .view()
                    .map(move |msg| KBleutoothMsg::BluetoothSettingsMsg(msg))
            } else {
                Space::with_width(Length::Shrink).into()
            });
        let inner_container = Container::new(embbeded_layout)
            .style(ContainerStyle::White)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10);
        Container::new(inner_container)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerStyle::LightGray)
            .into()
    }
}

#[derive(Default, Debug, Clone)]
pub struct BluetoothSettings {
    connected_host: text_input::State,
    connected_host_val: String,
    disconn_btn: button::State,
    ignore_dev: button::State,
    send_file: button::State,
    hide_btn: button::State,
}
#[derive(Debug, Clone)]
pub enum BluetoothSettingsMsg {
    HostNameChanged(String),
    Disconnected,
    Ignoranced,
    SendFile,
    HideSettings,
    SubmitChanged,
}

impl BluetoothSettings {
    fn new() -> Self {
        Self {
            connected_host_val: String::from("sna-koompi"),
            ..Default::default()
        }
    }
    fn update(&mut self, msg: BluetoothSettingsMsg) {
        use BluetoothSettingsMsg::*;
        match msg {
            HostNameChanged(val) => self.connected_host_val = val,
            Disconnected => {}
            Ignoranced => {}
            SendFile => {}
            HideSettings => {}
            SubmitChanged => {}
        }
    }
    fn view(&mut self) -> Element<BluetoothSettingsMsg> {
        let BluetoothSettings {
            connected_host,
            connected_host_val,
            ..
        } = self;
        let blue_settings_layout = Column::new()
            .spacing(10)
            .padding(10)
            .push(
                Button::new(&mut self.hide_btn, Icon::new('\u{f104}'))
                    .on_press(BluetoothSettingsMsg::HideSettings)
                    .style(ButtonStyle::Circular(86, 101, 115, 1.0)),
            )
            .push(
                Column::new()
                    .align_items(Align::Center)
                    .width(Length::Fill)
                    .push(Text::new("Connected Host Bluetooth").size(16)),
            )
            .push(
                TextInput::new(
                    connected_host,
                    connected_host_val,
                    "",
                    BluetoothSettingsMsg::HostNameChanged,
                )
                .on_submit(BluetoothSettingsMsg::SubmitChanged)
                .size(16)
                .width(Length::FillPortion(1))
                .padding(6)
                .style(InputStyle::InkBorder),
            )
            .push(
                Button::new(
                    &mut self.disconn_btn,
                    Text::new("Disconnect")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Disconnected),
            )
            .push(
                Button::new(
                    &mut self.ignore_dev,
                    Text::new("Ignore this device")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::Ignoranced),
            )
            .push(
                Button::new(
                    &mut self.send_file,
                    Text::new("Send Files")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Fill)
                .style(ButtonStyle::Circular(86, 101, 115, 1.0))
                .on_press(BluetoothSettingsMsg::SendFile),
            )
            .width(Length::Fill)
            .height(Length::Fill);
        Container::new(blue_settings_layout)
            .center_x()
            .center_y()
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .into()
    }
}

pub fn init() -> iced::Result {
    KBleutooth::run(Settings::default())
}
fn main() {
    match init() {
        Ok(()) => println!("run sucessfully"),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Hello World from Koompi Bluetooth");
}
