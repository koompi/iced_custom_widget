#[macro_use]
pub mod components;
pub mod examples;
pub mod styles;
pub mod utils;

use examples::menu::Menu;
use examples::table_demo::TableDemo;
use examples::system_setting::SystemSetting;

fn main() -> iced::Result {
   // TableDemo::init()
   // Menu::init()
   SystemSetting::init()
}
