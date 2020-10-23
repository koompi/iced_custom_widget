pub mod components;
pub mod examples;
pub mod styles;

use examples::menu::Menu;
use examples::card_test::CardTest;
fn main() -> iced::Result {
    Menu::init()
    // CardTest::init()
}
