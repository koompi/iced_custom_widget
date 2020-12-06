use iced::{pick_list, Background, Color};
pub enum CustomSelect {
    Default,
}
use iced_style::menu;
impl pick_list::StyleSheet for CustomSelect {
    fn menu(&self) -> menu::Style {
        menu::Style {
            ..Default::default()
        }
    }
    fn active(&self) -> pick_list::Style {
        match self {
            CustomSelect::Default => pick_list::Style {
                text_color: Color::WHITE,
                border_radius: 3.0,
                border_width: 1.0,
                icon_size: 0.5,
                border_color: Color::from_rgb8(119, 140, 163),
                background: Background::Color(Color::from_rgb8(165, 177, 194)),
            },
        }
    }
    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Background::Color(Color::from_rgb8(75, 101, 132)),
            ..self.active()
        }
    }
}
