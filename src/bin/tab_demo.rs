use iced::{
    button, executor, pick_list, scrollable, text_input, Align, Application, Button, Column,
    Command, Container, Element, Length, PickList, Row, Rule, Scrollable, Settings, Space, Text,
    TextInput,
};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::{
    buttons::ButtonStyle, containers::ContainerStyle, pick_list::PickListStyle, rules::RuleStyle,
    text_input::InputStyle,
};
use std::fmt;
/// # use iced_native::{renderer::Null, Element, Grid as NativeGrid, Text};
fn main() {
    init();
}
#[derive(Debug)]
pub struct TabContent {
    choice: Choice,
    wireless: Wireless,
    wire: Wire,
    network: NetSettings,
    is_active: bool,
}
impl Default for TabContent {
    fn default() -> Self {
        Self {
            choice: Choice::A,
            wireless: Wireless::default(),
            wire: Wire::new(),
            network: NetSettings::new(),
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
#[derive(Debug, Clone)]
pub enum AppMessage {
    TabSelect(Choice),
    WirelessMsg(WirelessMsg),
    WireMsg(WireMsg),
    ToggleChange(bool),
    NetSettingsMsg(NetSettingsMsg),
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
            AppMessage::NetSettingsMsg(msg) => {
                self.network.update(msg);
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
                Choice::B => self
                    .wireless
                    .view()
                    .map(move |msg| AppMessage::WirelessMsg(msg)),
                Choice::C => self
                    .network
                    .view()
                    .map(move |msg| AppMessage::NetSettingsMsg(msg)),
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
                    .style(ButtonStyle::BigCircular)
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
    // Applicaiton State
    general: (String, bool),
    commonip: CommonIp,
    security: Security,
    wlan: Wlan,
    // Application Ui State
    // Security Part
    host_name: text_input::State,
    host_value: String,
    is_auto_conn: bool,
    is_shown_passwd: bool,
    is_shown_private_key: bool,
    passwd: String,
    passwd_name: text_input::State,
    pick_list: pick_list::State<SecurityType>,
    pick_list1: pick_list::State<PwdOption>,
    pick_list3: pick_list::State<Authentication>,
    pick_list4: pick_list::State<EAPAuth>,
    selected_eapauth: EAPAuth,
    selected_auth: Authentication,
    selected_security: SecurityType,
    selected_pwdoption: PwdOption,
    toggle_show_passwd: button::State,
    identity: text_input::State,
    identity_val: String,
    private_pwd: text_input::State,
    private_pwd_val: String,
    private_key: text_input::State,
    private_key_val: String,
    ca_cert: text_input::State,
    ca_cert_val: String,
    user_cert: text_input::State,
    user_cert_val: String,
    private_pwd_file: button::State,
    private_key_file: button::State,
    ca_cert_file: button::State,
    user_cert_file: button::State,
    net_settings_scrolls: scrollable::State,
}

enum IP {
    IPV4,
    IPV6,
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

#[derive(Debug, Clone)]
pub enum NetSettingsMsg {
    HostChanged(String),
    HostSubmit,
    AutoConnMutated(bool),
    LanguageSelected(SecurityType),
    PwdOptionSelected(PwdOption),
    PasswordInput(String),
    ToggleShownPasswd,
    AuthChanged(Authentication),
    EAPAuthChanged(EAPAuth),
    IdentityChanged(String),
    PrivatePwdChanged(String),
    PrivateKeyChanged(String),
    CaCertChanged(String),
    UserCertChanged(String),
    ToggleKey,
    OpenFile1,
    OpenFile2,
    OpenFile3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityType {
    NONE,
    WEP,
    WPA_WPA2_PERSONAL,
    WPA_WPA2_ENTERPRISE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authentication {
    SharedKey,
    OpenSystem,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PwdOption {
    OneUser,
    AllUser,
    AskFirst,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EAPAuth {
    TLS,
    LEAP,
    FAST,
    TUNNELEDTLS,
    PROTECTEDEAP,
}
impl EAPAuth {
    const ALL: [EAPAuth; 5] = [
        EAPAuth::TLS,
        EAPAuth::LEAP,
        EAPAuth::FAST,
        EAPAuth::TUNNELEDTLS,
        EAPAuth::PROTECTEDEAP,
    ];
}
impl Default for EAPAuth {
    fn default() -> Self {
        EAPAuth::TLS
    }
}
impl fmt::Display for EAPAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EAPAuth::TLS => "TLS",
                EAPAuth::LEAP => "LEAP",
                EAPAuth::FAST => "FAST",
                EAPAuth::TUNNELEDTLS => "Tunneled TLS",
                EAPAuth::PROTECTEDEAP => "Protected EAP",
            }
        )
    }
}
impl Authentication {
    const ALL: [Authentication; 2] = [Authentication::SharedKey, Authentication::OpenSystem];
}
impl Default for Authentication {
    fn default() -> Self {
        Authentication::SharedKey
    }
}
impl fmt::Display for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Authentication::SharedKey => "ShareKey",
                Authentication::OpenSystem => "OpenSystem",
            }
        )
    }
}
impl PwdOption {
    const ALL: [PwdOption; 3] = [PwdOption::OneUser, PwdOption::AllUser, PwdOption::OneUser];
}
impl Default for PwdOption {
    fn default() -> Self {
        PwdOption::OneUser
    }
}
impl fmt::Display for PwdOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PwdOption::OneUser => "Save password for this user",
                PwdOption::AllUser => "Save password for all users",
                PwdOption::AskFirst => "Ask me always",
            }
        )
    }
}
impl SecurityType {
    const ALL: [SecurityType; 4] = [
        SecurityType::NONE,
        SecurityType::WEP,
        SecurityType::WPA_WPA2_PERSONAL,
        SecurityType::WPA_WPA2_ENTERPRISE,
    ];
}

impl Default for SecurityType {
    fn default() -> Self {
        SecurityType::WPA_WPA2_PERSONAL
    }
}
impl std::fmt::Display for SecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SecurityType::NONE => "None",
                SecurityType::WEP => "WEP",
                SecurityType::WPA_WPA2_PERSONAL => "WPA/WPA2 Personal",
                SecurityType::WPA_WPA2_ENTERPRISE => "WPA/WPA2 Enterprise",
            }
        )
    }
}

fn default_text(text: &str) -> Text {
    Text::new(text).size(16)
}
impl NetSettings {
    fn new() -> Self {
        Self {
            is_shown_passwd: true,
            ..Self::default()
        }
    }

    fn update(&mut self, msg: NetSettingsMsg) {
        match msg {
            NetSettingsMsg::HostChanged(name) => {
                self.host_value = name;
            }
            NetSettingsMsg::HostSubmit => {}
            NetSettingsMsg::AutoConnMutated(is_auto_conn) => {
                self.is_auto_conn = is_auto_conn;
            }
            NetSettingsMsg::LanguageSelected(lang) => {
                self.selected_security = lang;
            }
            NetSettingsMsg::PwdOptionSelected(opts) => {
                self.selected_pwdoption = opts;
            }
            NetSettingsMsg::PasswordInput(password) => {
                self.passwd = password;
            }
            NetSettingsMsg::ToggleShownPasswd => {
                self.is_shown_passwd = !self.is_shown_passwd;
            }
            NetSettingsMsg::AuthChanged(auth) => {
                self.selected_auth = auth;
            }
            NetSettingsMsg::EAPAuthChanged(eapauth) => {
                self.selected_eapauth = eapauth;
            }
            _ => {}
        }
    }
    fn view(&mut self) -> Element<NetSettingsMsg> {
        let NetSettings {
            host_name,
            host_value,
            is_auto_conn,
            passwd,
            passwd_name,
            identity,
            identity_val,
            private_pwd,
            private_pwd_val,
            private_key,
            private_key_val,
            ca_cert,
            ca_cert_val,
            user_cert,
            user_cert_val,
            private_pwd_file,
            private_key_file,
            ca_cert_file,
            user_cert_file,
            net_settings_scrolls,
            ..
        } = self;
        let net_layout = Column::new();
        let general = Column::new()
            .padding(10)
            .spacing(10)
            .push(Text::new("General").size(24))
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .padding(10)
                        .push(
                            Row::new()
                                .push(default_text("Name").width(Length::FillPortion(1)))
                                .align_items(Align::Center)
                                .push(
                                    TextInput::new(
                                        host_name,
                                        "Koompi Attic",
                                        host_value,
                                        NetSettingsMsg::HostChanged,
                                    )
                                    .padding(6)
                                    .style(InputStyle::InkBorder)
                                    .width(Length::FillPortion(2))
                                    .on_submit(NetSettingsMsg::HostSubmit),
                                ),
                        )
                        .push(Rule::horizontal(4).style(RuleStyle {}))
                        .push(
                            Row::new()
                                .push(default_text("Auto Connect"))
                                .push(Space::with_width(Length::Fill))
                                .push(Toggler::new(
                                    *is_auto_conn,
                                    "".to_string(),
                                    NetSettingsMsg::AutoConnMutated,
                                )),
                        ),
                )
                .width(Length::Fill)
                .style(ContainerStyle::LightGrayCircle),
            );
        let security = Column::new()
            .padding(10)
            .spacing(10)
            .push(Text::new("Security").size(24))
            .push(
                Container::new(
                    Column::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .padding(10)
                        .push(
                            Row::new()
                                .push(default_text("Security").width(Length::FillPortion(1)))
                                .align_items(Align::Center)
                                .push(
                                    PickList::new(
                                        &mut self.pick_list,
                                        &SecurityType::ALL[..],
                                        Some(self.selected_security),
                                        NetSettingsMsg::LanguageSelected,
                                    )
                                    .text_size(16)
                                    .style(PickListStyle {})
                                    .padding(6)
                                    .width(Length::FillPortion(2)),
                                )
                                .width(Length::FillPortion(3)),
                        )
                        .push(match self.selected_security {
                            SecurityType::NONE => {
                                Container::new(Space::with_height(Length::Units(0)))
                            }
                            SecurityType::WEP => Container::new(
                                Column::new()
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(default_text("Key").width(Length::FillPortion(1)))
                                            .spacing(4)
                                            .push(if self.is_shown_passwd {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    &mut self.toggle_show_passwd,
                                                    Icon::new(if self.is_shown_passwd {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .style(ButtonStyle::Circular)
                                                .on_press(NetSettingsMsg::ToggleShownPasswd),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(
                                                default_text("Authentication")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list3,
                                                    &Authentication::ALL[..],
                                                    Some(self.selected_auth),
                                                    NetSettingsMsg::AuthChanged,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    ),
                            ),
                            SecurityType::WPA_WPA2_PERSONAL => Container::new(
                                Column::new()
                                    .spacing(10)
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(Rule::horizontal(10).style(RuleStyle {}))
                                    .push(
                                        Row::new()
                                            .align_items(Align::Center)
                                            .push(
                                                default_text("Password")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .spacing(4)
                                            .push(if self.is_shown_passwd {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    passwd_name,
                                                    "Required",
                                                    passwd,
                                                    NetSettingsMsg::PasswordInput,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    &mut self.toggle_show_passwd,
                                                    Icon::new(if self.is_shown_passwd {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .style(ButtonStyle::Circular)
                                                .on_press(NetSettingsMsg::ToggleShownPasswd),
                                            ),
                                    ),
                            ),
                            SecurityType::WPA_WPA2_ENTERPRISE => Container::new(
                                Column::new()
                                    .padding(10)
                                    .spacing(10)
                                    .push(
                                        Row::new().push(
                                            PickList::new(
                                                &mut self.pick_list4,
                                                &EAPAuth::ALL[..],
                                                Some(self.selected_eapauth),
                                                NetSettingsMsg::EAPAuthChanged,
                                            )
                                            .text_size(16)
                                            .style(PickListStyle {})
                                            .padding(6)
                                            .width(Length::FillPortion(2)),
                                        ),
                                    )
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Pwd Options")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .align_items(Align::Center)
                                            .push(
                                                PickList::new(
                                                    &mut self.pick_list1,
                                                    &PwdOption::ALL[..],
                                                    Some(self.selected_pwdoption),
                                                    NetSettingsMsg::PwdOptionSelected,
                                                )
                                                .text_size(16)
                                                .style(PickListStyle {})
                                                .padding(6)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .push(
                                                default_text("Identity")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    identity,
                                                    "Required",
                                                    identity_val,
                                                    NetSettingsMsg::IdentityChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("Private Pwd")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(if self.is_shown_private_key {
                                                TextInput::new(
                                                    private_pwd,
                                                    "Required",
                                                    private_pwd_val,
                                                    NetSettingsMsg::PrivatePwdChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            } else {
                                                TextInput::new(
                                                    private_pwd,
                                                    "Required",
                                                    private_pwd_val,
                                                    NetSettingsMsg::PrivatePwdChanged,
                                                )
                                                .password()
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2))
                                            })
                                            .push(
                                                Button::new(
                                                    &mut self.private_pwd_file,
                                                    Icon::new(if self.is_shown_private_key {
                                                        '\u{f06e}'
                                                    } else {
                                                        '\u{f070}'
                                                    }),
                                                )
                                                .on_press(NetSettingsMsg::ToggleKey),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("Private Key")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    private_key,
                                                    "",
                                                    private_key_val,
                                                    NetSettingsMsg::PrivateKeyChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(
                                                    private_key_file,
                                                    Icon::new('\u{f0c6}'),
                                                )
                                                .on_press(NetSettingsMsg::OpenFile1)
                                                .style(ButtonStyle::Circular),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("CA Cert")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    ca_cert,
                                                    "",
                                                    ca_cert_val,
                                                    NetSettingsMsg::CaCertChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(ca_cert_file, Icon::new('\u{f0c6}'))
                                                    .on_press(NetSettingsMsg::OpenFile2)
                                                    .style(ButtonStyle::Circular),
                                            ),
                                    )
                                    .push(
                                        Row::new()
                                            .spacing(4)
                                            .push(
                                                default_text("User Cert")
                                                    .width(Length::FillPortion(1)),
                                            )
                                            .push(
                                                TextInput::new(
                                                    user_cert,
                                                    "",
                                                    user_cert_val,
                                                    NetSettingsMsg::UserCertChanged,
                                                )
                                                .padding(6)
                                                .style(InputStyle::InkBorder)
                                                .width(Length::FillPortion(2)),
                                            )
                                            .push(
                                                Button::new(user_cert_file, Icon::new('\u{f0c6}'))
                                                    .on_press(NetSettingsMsg::OpenFile3)
                                                    .style(ButtonStyle::Circular),
                                            ),
                                    ),
                            )
                            .width(Length::Fill),
                        }),
                )
                .style(ContainerStyle::LightGrayCircle)
                .width(Length::Fill),
            );
        // let ipv4 = Container::new(Column::new().push(child: E))
        let network_scroll =
            Scrollable::new(net_settings_scrolls).push(net_layout.push(general).push(security));
        Container::new(network_scroll)
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
