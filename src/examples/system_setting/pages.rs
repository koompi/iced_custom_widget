mod general_page;
mod home_page;

use general_page::{general_page, Hightight, IconSize};
use home_page::home_page;
use iced::Element;
use iced::{button, pick_list};

pub struct Pages {
    pages: Vec<PageModel>,
    current: usize,
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
    CheckboxToggle(bool),
    SelectIconSize(IconSize),
    SelectColor(Hightight),
    DarkButton,
    LightButton,
}

#[derive(Debug, Clone)]
pub enum PageModel {
    HomePage,
    GeneralPage {
        hide_show_menubar: bool,
        selected_list1: IconSize,
        selected_list2: Hightight,
        light_btn: button::State,
        dark_btn: button::State,
        icon_size: pick_list::State<IconSize>,
        highlight: pick_list::State<Hightight>,
    },
    DateTimePage,
    LanguagePage,
    UsersPage,
    AccessPage,
    AccountPage,
    NotiPage,
    SecurityPage,
    UpdatePage,
    NetworkPage,
    BluetoothPage,
    SoundPage,
    PrinterPage,
    CameraPage,
    KeyboardPage,
    TouchpadPage,
    MicPage,
    MousePage,
    DisplayPage,
    BatteryPage,
    DiskDrivePage,
}

impl Pages {
    pub fn new() -> Self {
        use PageModel::*;
        Self {
            pages: vec![
                HomePage,
                GeneralPage {
                    hide_show_menubar: false,
                    selected_list1: IconSize::default(),
                    selected_list2: Hightight::default(),
                    light_btn: button::State::new(),
                    dark_btn: button::State::new(),
                    icon_size: pick_list::State::default(),
                    highlight: pick_list::State::default(),
                },
                DateTimePage,
                LanguagePage,
                UsersPage,
                AccessPage,
                AccountPage,
                NotiPage,
                SecurityPage,
                UpdatePage,
                NetworkPage,
                BluetoothPage,
                SoundPage,
                PrinterPage,
                CameraPage,
                KeyboardPage,
                TouchpadPage,
                MicPage,
                MousePage,
                DisplayPage,
                BatteryPage,
                DiskDrivePage,
            ],
            current: 0,
        }
    }

    pub fn set_current(&mut self, idx: usize) {
        self.current = idx;
    }

    pub fn update(&mut self, msg: PagesMessage) {
        self.pages[self.current].update(msg);
    }

    pub fn view(&mut self) -> Element<PagesMessage> {
        self.pages[self.current].view()
    }

    pub fn title(&self) -> &str {
        self.pages[self.current].title()
    }
}

impl PageModel {
    fn update(&mut self, msg: PagesMessage) {
        match msg {
            PagesMessage::CheckboxToggle(b) => {
                if let PageModel::GeneralPage {
                    hide_show_menubar, ..
                } = self
                {
                    *hide_show_menubar = b;
                }
            }
            PagesMessage::SelectColor(color) => {
                if let PageModel::GeneralPage { selected_list2, .. } = self {
                    *selected_list2 = color;
                }
            }
            PagesMessage::SelectIconSize(size) => {
                if let PageModel::GeneralPage { selected_list1, .. } = self {
                    *selected_list1 = size;
                }
            }
            PagesMessage::DarkButton => {}
            PagesMessage::LightButton => {}
        }
    }

    fn view(&mut self) -> Element<PagesMessage> {
        use PageModel::*;
        match self {
            HomePage => home_page(),
            GeneralPage {
                hide_show_menubar,
                selected_list1,
                selected_list2,
                light_btn,
                dark_btn,
                icon_size,
                highlight,
            } => general_page(
                *hide_show_menubar,
                *selected_list1,
                *selected_list2,
                light_btn,
                dark_btn,
                icon_size,
                highlight,
            ),
            DateTimePage => home_page(),
            LanguagePage => home_page(),
            UsersPage => home_page(),
            AccessPage => home_page(),
            AccountPage => home_page(),
            NotiPage => home_page(),
            SecurityPage => home_page(),
            UpdatePage => home_page(),
            NetworkPage => home_page(),
            BluetoothPage => home_page(),
            SoundPage => home_page(),
            PrinterPage => home_page(),
            CameraPage => home_page(),
            KeyboardPage => home_page(),
            TouchpadPage => home_page(),
            MicPage => home_page(),
            MousePage => home_page(),
            DisplayPage => home_page(),
            BatteryPage => home_page(),
            DiskDrivePage => home_page(),
        }
        .into()
    }

    fn title(&self) -> &str {
        use PageModel::*;
        match self {
            HomePage => "System Setting",
            GeneralPage { .. } => "General",
            DateTimePage => "Date & Time",
            LanguagePage => "Language & Region",
            UsersPage => "Users & Groups",
            AccessPage => "Accessibility",
            AccountPage => "Accounts",
            NotiPage => "Notifications",
            SecurityPage => "Security & Privacy",
            UpdatePage => "Software Update",
            NetworkPage => "Network",
            BluetoothPage => "Bluetooth",
            SoundPage => "Sound",
            PrinterPage => "Printers & Scanners",
            CameraPage => "Camera",
            KeyboardPage => "Keyboard",
            TouchpadPage => "Touchpad",
            MicPage => "Microphone",
            MousePage => "Mouse",
            DisplayPage => "Display",
            BatteryPage => "Battery",
            DiskDrivePage => "Disk Drive",
        }
    }
}
