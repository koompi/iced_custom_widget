use crate::styles::custom_styles::CustomButton;
use crate::styles::custom_styles::{CustomContainer, CustomTextInput};
use crate::utils::themes::Theme;
use iced::{
    button, pick_list, Align, Button, Checkbox, Column, Container, HorizontalAlignment, Image,
    Length, PickList, Row, Rule, Space, Text,
};
use std::fmt;
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum Hightight {
    Blue,
    Green,
    Red,
    Yellow,
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
pub enum IconSize {
    Small,
    Medium,
    Big,
    Large,
}
impl Hightight {
    const ALL: [Hightight; 4] = [
        Hightight::Blue,
        Hightight::Green,
        Hightight::Red,
        Hightight::Yellow,
    ];
}
impl fmt::Display for Hightight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hightight::Blue => "Blue",
                Hightight::Green => "Green",
                Hightight::Red => "Red",
                Hightight::Yellow => "Yellow",
            }
        )
    }
}

impl fmt::Display for IconSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconSize::Small => "Small",
                IconSize::Medium => "Medium",
                IconSize::Big => "Big",
                IconSize::Large => "Large",
            }
        )
    }
}
impl Default for IconSize {
    fn default() -> Self {
        IconSize::Medium
    }
}
impl Default for Hightight {
    fn default() -> Self {
        Hightight::Blue
    }
}
impl IconSize {
    const ALL: [IconSize; 4] = [
        IconSize::Small,
        IconSize::Medium,
        IconSize::Big,
        IconSize::Large,
    ];
}
use super::super::pages::PagesMessage;

pub fn general_page<'a>(
    checkbox: bool,
    select1: IconSize,
    select2: Hightight,
    light: &'a mut button::State,
    dark: &'a mut button::State,
    list1: &'a mut pick_list::State<IconSize>,
    list2: &'a mut pick_list::State<Hightight>,
) -> Container<'a, PagesMessage> {
    let main_layout = Row::new();
    let first_col = Column::new()
        .push(Space::with_height(Length::Units(50)))
        .push(Text::new("Appearance: "))
        .push(Space::with_height(Length::Units(50)))
        .align_items(Align::End)
        .push(Text::new("Hightight Color"))
        .push(Space::with_height(Length::Units(5)))
        .push(Text::new("Sidebar Icon Size"));
    let checkbox = Checkbox::new(
        checkbox,
        "Automatically hide and show the menu bar",
        PagesMessage::CheckboxToggle,
    );
    let last_col = Column::new()
        .push(
            Row::new()
                .spacing(16)
                .push(
                    Column::new()
                        .align_items(Align::Center)
                        .push(
                            Button::new(
                                light,
                                Image::new(format!(
                                    "{}/assets/images/battery.svg",
                                    env!("CARGO_MANIFEST_DIR")
                                )),
                            )
                            .on_press(PagesMessage::LightButton)
                            .min_width(100)
                            .min_height(50)
                            .style(CustomButton::Selected(Theme::light().palette)),
                        )
                        .spacing(5)
                        .push(Text::new("Light").horizontal_alignment(HorizontalAlignment::Center)),
                )
                .push(
                    Column::new()
                        .push(
                            Button::new(
                                dark,
                                Image::new(format!(
                                    "{}/assets/images/battery.svg",
                                    env!("CARGO_MANIFEST_DIR")
                                )),
                            )
                            .on_press(PagesMessage::DarkButton)
                            .min_width(100)
                            .min_height(50)
                            .style(CustomButton::Selected(Theme::dark().palette)),
                        )
                        .spacing(5)
                        .push(Text::new("Dark").horizontal_alignment(HorizontalAlignment::Center)),
                ),
        )
        .push(
            PickList::new(
                list1,
                &IconSize::ALL[..],
                Some(select1),
                PagesMessage::SelectIconSize,
            )
            .width(Length::Units(150)),
        )
        .spacing(15)
        .align_items(Align::Start)
        .push(
            PickList::new(
                list2,
                &Hightight::ALL[..],
                Some(select2),
                PagesMessage::SelectColor,
            )
            .width(Length::Units(150)),
        )
        .push(checkbox);

    Container::new(main_layout.push(first_col).push(last_col).spacing(10))
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .style(CustomContainer::FadedBrightForeground(
            Theme::light().palette,
        ))
}
