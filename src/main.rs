pub mod components;
pub mod examples;
pub mod styles;

use examples::card_test::CardTest;
use examples::menu::Menu;

fn main() {
    Menu::init().unwrap();
    // CardTest::init().unwrap();
}
