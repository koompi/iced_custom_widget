pub mod components;
pub mod examples;
pub mod styles;
pub mod utils;

use examples::card_test::CardTest;
use examples::menu::Menu;
use examples::stack_demo::StackDemo;

fn main() -> iced::Result {
    Menu::init()
    // CardTest::init()
    // StackDemo::init()
}
