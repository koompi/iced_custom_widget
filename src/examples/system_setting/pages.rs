mod home_page;
mod general_page;

use home_page::home_page;
use general_page::general_page;
use iced::Element; 

pub struct Pages {
   pages: Vec<PageModel>,
   current: usize
}

#[derive(Debug, Clone)]
pub enum PagesMessage {
   CheckboxToggle(bool)
}

#[derive(Debug, Clone)]
pub enum PageModel {
   HomePage,
   GeneralPage {
      checkbox: bool
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
   DiskDrivePage
}

impl Pages {
   pub fn new() -> Self {
      use PageModel::*;
      Self {
         pages: vec![
            HomePage,
            GeneralPage {
               checkbox: false
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
            DiskDrivePage
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
            if let PageModel::GeneralPage{checkbox} = self {
               *checkbox = b;
            }
         }
      }
   }

   fn view(&mut self) -> Element<PagesMessage> {
      use PageModel::*;
      match self {
         HomePage => home_page(),
         GeneralPage { checkbox } => general_page(*checkbox),
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
         TouchpadPage=> home_page(),
         MicPage=> home_page(),
         MousePage=> home_page(),
         DisplayPage=> home_page(),
         BatteryPage=> home_page(),
         DiskDrivePage => home_page()
      }.into()
   }

   fn title(&self) -> &str {
      use PageModel::*;
      match self {
         HomePage => "System Setting",
         GeneralPage{..} => "General",
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
         TouchpadPage=> "Touchpad",
         MicPage=> "Microphone",
         MousePage=> "Mouse",
         DisplayPage=> "Display",
         BatteryPage=> "Battery",
         DiskDrivePage => "Disk Drive"
      }
   }
}