#[macro_use]
pub mod components;
pub mod examples;
pub mod styles;
pub mod utils;

// extern crate components;
use examples::menu::Menu;
use examples::table_demo::TableDemo;

fn main() {
   TableDemo::init();
   // Menu::init();
}
