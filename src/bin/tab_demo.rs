use iced::{
    button, executor, Align, Application, Button, Column, Command, Container, Element, Length, Row,
    Rule, Settings, Space, Text,
};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::containers::ContainerStyle;
/// # use iced_native::{renderer::Null, Element, Grid as NativeGrid, Text};
fn main() {
    init();
}
#[derive(Debug)]
pub struct TabContent {
    choice: Choice,
    wireless: Wireless,
    wire: Wire,
    is_active: bool,
}
impl Default for TabContent {
    fn default() -> Self {
        Self {
            choice: Choice::A,
            wireless: Wireless::default(),
            wire: Wire::new(),
            is_active: false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
#[derive(Debug, Copy, Clone)]
pub enum AppMessage {
    TabSelect(Choice),
    WirelessMsg(WirelessMsg),
    WireMsg(WireMsg),
    ToggleChange(bool),
}

impl Application for TabContent {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMessage;

    fn title(&self) -> String {
        String::from("GridView Applicaiton ")
    }
    fn new(_flags: ()) -> (TabContent, Command<AppMessage>) {
        (TabContent::default(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::TabSelect(select) => {
                self.choice = select;
                Command::none()
            }
            AppMessage::WirelessMsg(msg) => {
                self.wireless.update(msg);
                Command::none()
            }
            AppMessage::ToggleChange(is_active) => {
                self.is_active = is_active;
                Command::none()
            }
            AppMessage::WireMsg(msg) => {
                self.wire.update(msg);
                Command::none()
            }
        }
    }
    fn view(&mut self) -> Element<Self::Message> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Tab::new(
                    Choice::A,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f796}', "Ethernet"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::B,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f1eb}', "Wireless"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::C,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f6ff}', "DSL"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::D,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f3ed}', "VPN"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::E,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f7ba}', "System Proxy"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::F,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f7b9}', "Application Proxy"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::G,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f0c1}', "Personal Hotspot"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::H,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    tab_content('\u{f05a}', "Network Details"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            );
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(10)
            .push(match self.choice {
                Choice::A => self.wire.view().map(move |msg| AppMessage::WireMsg(msg)),
                Choice::B => {
                    // Row::new().push(Text::new("Network Adapter Wireless")).push(Toggler::new(self.is_active, String::from("Active"), Conmsg::ToggleChange)
                    // .width(Length::Shrink))
                    self.wireless
                        .view()
                        .map(move |msg| AppMessage::WirelessMsg(msg))
                }
                Choice::C => Text::new("Content C").into(),
                Choice::D => Text::new("Content D").into(),
                Choice::F => Text::new("Content F").into(),
                Choice::G => Text::new("Content G").into(),
                Choice::E => Text::new("Content E").into(),
                Choice::H => Text::new("Content H").into(),
            });
        let whole_content: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(row.height(Length::Fill))
                    .style(ContainerStyle::White)
                    .width(Length::FillPortion(4))
                    .height(Length::Fill)
                    .padding(15),
            )
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill))
                    .width(Length::FillPortion(9))
                    .height(Length::Fill)
                    .style(ContainerStyle::White)
                    .padding(20),
            )
            .into();
        let container = Container::new(whole_content)
            .width(Length::Fill)
            .center_x()
            .center_y();
        Container::new(container)
            .style(ContainerStyle::LightGray)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
fn tab_content(unicode: char, name: &str) -> Row<'static, AppMessage> {
    Row::new()
        .push(Icon::new(unicode).size(24))
        .push(Text::new(name).size(16))
        .align_items(Align::Center)
        .spacing(8)
}
pub fn init() {
    match TabContent::run(Settings::default()) {
        Ok(val) => println!("run success with exit code: {:?}", val),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[derive(Default, Debug)]
pub struct Wireless {
    is_active: bool,
    status: String,
    security: Option<String>,
    ssid: String,
}

impl Wireless {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, msg: WirelessMsg) {
        match msg {
            WirelessMsg::EnableWireless(value) => {
                self.is_active = value;
            }
        }
    }
    pub fn view(&mut self) -> Element<WirelessMsg> {
        Toggler::new(
            self.is_active,
            String::from("Wireless Network Adapter"),
            WirelessMsg::EnableWireless,
        )
        .into()
    }
}
#[derive(Debug, Copy, Clone)]
pub enum WirelessMsg {
    EnableWireless(bool),
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Wire {
    is_enable: bool,
    add_net_con: button::State,
}
#[derive(Debug, Copy, Clone)]
pub enum WireMsg {
    EnableWired(bool),
    NetworkAdded,
}
use icw::styles::custom_button::CustomButton;
impl Wire {
    fn new() -> Self {
        Self { ..Self::default() }
    }
    fn update(&mut self, msg: WireMsg) {
        match msg {
            WireMsg::EnableWired(is_enable) => {
                self.is_enable = is_enable;
            }
            WireMsg::NetworkAdded => {}
        }
    }
    fn view(&mut self) -> Element<WireMsg> {
        Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(Toggler::new(
                self.is_enable,
                String::from("Wire Network Adapter"),
                WireMsg::EnableWired,
            ))
            .push(
                Container::new(Text::new("Plug in the network cable first"))
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Units(100))
                    .style(ContainerStyle::LightGrayCircle),
            )
            .push(Space::with_height(Length::Fill))
            .push(
                Button::new(&mut self.add_net_con, Icon::new('\u{f067}').size(24))
                    .style(CustomButton::Circular)
                    .padding(10)
                    .width(Length::Units(50))
                    .height(Length::Units(50))
                    .on_press(WireMsg::NetworkAdded),
            )
            .into()
    }
}
#[derive(Default, Debug, Clone)]
pub struct NetSettings {
    general: (String, bool),
    commonip: CommonIp,
    security: Security,
    wlan: Wlan,
}
enum IP {
    Ipv4,
    Ipv6,
}
#[derive(Default, Debug, Clone)]
struct CommonIp {
    Method: Vec<String>,
    PrimaryDns: String,
    SecondaryDns: String,
}
#[derive(Default, Debug, Clone)]
struct Security {
    Type: Vec<String>,
    PwdOption: Vec<u8>,
    Pwd: String,
}
#[derive(Default, Debug, Clone)]
struct Wlan {
    Ssid: String,
    DevMacAddr: Option<String>,
    CustomMtu: bool,
}

impl NetSettings {
    fn new() -> Self {
        Self { ..Self::default() }
    }
}
