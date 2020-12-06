mod general_page;

use general_page::{
    ColorAccent, General, GeneralMessage, Hightight, IconSize, ShowSrollBar, WebBrowsers,
};
use iced::Element;
use iced::{button, pick_list, Container, Length, Space};

pub struct Pages {
    pages: Vec<PageModel>,
    current: usize,
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
    GeneralMessage(GeneralMessage),
}

#[derive(Debug, Clone)]
pub enum PageModel {
    HomePage,
    GeneralPage { general: General },
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
                    general: General::new(),
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
            PagesMessage::GeneralMessage(msg) => {
                if let PageModel::GeneralPage { general } = self {
                    general.update(msg);
                }
            }
        }
    }

    fn view(&mut self) -> Element<PagesMessage> {
        use PageModel::*;
        match self {
            HomePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            GeneralPage { general } => general
                .view()
                .map(move |msg| PagesMessage::GeneralMessage(msg)),
            DateTimePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            LanguagePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            UsersPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            AccessPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            AccountPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            NotiPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),

            SecurityPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),

            UpdatePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            NetworkPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            BluetoothPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            SoundPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            PrinterPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            CameraPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            KeyboardPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            TouchpadPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            MicPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            MousePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            DisplayPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            BatteryPage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
            DiskDrivePage => Container::new(Space::new(Length::Fill, Length::Fill)).into(),
        }
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
