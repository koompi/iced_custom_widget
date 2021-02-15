use crate::utils::ColorPalette;
use iced::{
   button, checkbox, container, pick_list, scrollable, text_input, Background, Color, Vector
};

pub enum CustomContainer {
   BrightForeground(ColorPalette),
   NormalForeground(ColorPalette),
   FadedNormalForeground(ColorPalette),
   FadedBrightForeground(ColorPalette),
   NormalBackground(ColorPalette),
   BrightBackground(ColorPalette),
   Segmented(ColorPalette),
   NormalErrorBackground(ColorPalette),
   NormalErrorForeground(ColorPalette),
}

impl container::StyleSheet for CustomContainer {
   fn style(&self) -> container::Style {
      container::Style {
         background: Some(Background::Color(match self {
            Self::BrightForeground(palette)
            | Self::NormalForeground(palette)
            | Self::NormalErrorForeground(palette) => palette.base.foreground,
            Self::FadedNormalForeground(palette) | Self::FadedBrightForeground(palette) => Color {
               a: 0.80,
               ..palette.base.foreground
            },
            Self::NormalBackground(palette)
            | Self::BrightBackground(palette)
            | Self::NormalErrorBackground(palette) => palette.base.background,
            Self::Segmented(_) => match container::Style::default()
               .background
               .unwrap_or(Background::Color(Color::TRANSPARENT))
            {
               Background::Color(color) => color,
            },
         })),
         text_color: Some(match self {
            Self::NormalForeground(palette)
            | Self::NormalBackground(palette)
            | Self::FadedNormalForeground(palette)
            | Self::NormalErrorForeground(palette)
            | Self::NormalErrorBackground(palette) => palette.normal.surface,
            Self::BrightForeground(palette)
            | Self::BrightBackground(palette)
            | Self::FadedBrightForeground(palette) => palette.bright.surface,
            Self::Segmented(_) => container::Style::default()
               .text_color
               .unwrap_or(Color::BLACK),
         }),
         // border_radius: match self {
         //    Self::Segmented(_) => 4.0,
         //    _ => container::Style::default().border_radius,
         // },
         border_radius: 4.0,
         border_width: match self {
            Self::Segmented(_) => 1.0,
            _ => container::Style::default().border_width,
         },
         border_color: match self {
            Self::Segmented(palette) => Color {
               a: 0.5,
               ..palette.normal.primary
            },
            _ => container::Style::default().border_color,
         },
      }
   }
}

pub enum CustomButton {
   Default(ColorPalette),
   DefaultBoxed(ColorPalette),
   DefaultDelete(ColorPalette),
   Disabled(ColorPalette),
   Selected(ColorPalette),
   Secondary(ColorPalette),
   SecondaryBoxed(ColorPalette),
   BrightText(ColorPalette),
   SelectedBrightText(ColorPalette),
}

impl button::StyleSheet for CustomButton {
   fn active(&self) -> button::Style {
      button::Style {
         text_color: match self {
            Self::BrightText(palette) => palette.bright.surface,
            Self::Default(palette)
            | Self::DefaultBoxed(palette)
            | Self::Selected(palette)
            | Self::SelectedBrightText(palette) => palette.bright.primary,
            Self::Secondary(palette) | Self::SecondaryBoxed(palette) => palette.bright.secondary,
            Self::DefaultDelete(palette) => palette.bright.error,
            Self::Disabled(palette) => Color {
               a: 0.25,
               ..palette.normal.surface
            },
         },
         background: Some(Background::Color(match self {
            Self::SecondaryBoxed(palette) => Color {
               a: 0.15,
               ..palette.normal.secondary
            },
            Self::Disabled(_) => Color::TRANSPARENT,
            Self::Selected(palette) => palette.normal.primary,
            _ => match button::Style::default()
               .background
               .unwrap_or(Background::Color(Color::TRANSPARENT))
            {
               Background::Color(color) => color,
            },
         })),
         border_radius: 5.0,
         border_color: match self {
            Self::DefaultBoxed(palette) => Color {
               a: 0.5,
               ..palette.normal.primary
            },
            _ => button::Style::default().border_color,
         },
         border_width: match self {
            Self::DefaultBoxed(_) => 1.0,
            _ => button::Style::default().border_width,
         },
         ..button::Style::default()
      }
   }

   fn hovered(&self) -> button::Style {
      button::Style {
         text_color: match self {
            Self::Default(palette)
            | Self::DefaultBoxed(palette)
            | Self::Selected(palette)
            | Self::BrightText(palette)
            | Self::SelectedBrightText(palette) => palette.bright.primary,
            Self::Secondary(palette) | Self::SecondaryBoxed(palette) => palette.bright.secondary,
            Self::DefaultDelete(palette) => palette.bright.error,
            Self::Disabled(_) => self.active().text_color,
         },
         background: Some(Background::Color(match self {
            Self::Default(palette) | Self::DefaultBoxed(palette) | Self::Selected(palette) => {
               palette.normal.primary
            }
            Self::Secondary(palette) | Self::SecondaryBoxed(palette) => palette.normal.secondary,
            Self::DefaultDelete(palette) => Color {
               a: 0.35,
               ..palette.normal.error
            },
            Self::Disabled(_) => match self
               .active()
               .background
               .unwrap_or(Background::Color(Color::TRANSPARENT))
            {
               Background::Color(color) => color,
            },
            _ => Color::TRANSPARENT,
         })),
         ..self.active()
      }
   }

   fn disabled(&self) -> button::Style {
      button::Style {
         text_color: match self {
            Self::SecondaryBoxed(palette) => Color {
               a: 0.15,
               ..palette.bright.secondary
            },
            Self::Default(palette) | Self::BrightText(palette) => Color {
               a: 0.25,
               ..palette.normal.surface
            },
            Self::DefaultBoxed(palette) | Self::SelectedBrightText(palette) => Color {
               a: 0.50,
               ..palette.bright.primary
            },
            Self::Secondary(palette) => Color {
               a: 0.70,
               ..palette.normal.secondary
            },
            Self::Selected(palette) => palette.bright.primary,
            Self::DefaultDelete(palette) => palette.bright.error,
            Self::Disabled(_) => self.active().text_color,
         },
         background: Some(Background::Color(match self {
            Self::DefaultBoxed(palette) => Color {
               a: 0.05,
               ..palette.normal.primary
            },
            Self::SecondaryBoxed(palette) => Color {
               a: 0.05,
               ..palette.normal.secondary
            },
            Self::Secondary(palette) => Color {
               a: 0.70,
               ..palette.normal.secondary
            },
            Self::BrightText(_) | Self::SelectedBrightText(_) | Self::DefaultDelete(_) => {
               match self
                  .active()
                  .background
                  .unwrap_or(Background::Color(Color::TRANSPARENT))
               {
                  Background::Color(color) => Color {
                     a: color.a * 0.5,
                     ..color
                  },
               }
            }
            Self::Default(_) | Self::Disabled(_) | Self::Selected(_) => {
               match self
                  .active()
                  .background
                  .unwrap_or(Background::Color(Color::TRANSPARENT))
               {
                  Background::Color(color) => color,
               }
            }
         })),
         ..self.active()
      }
   }
}

pub enum CustomScrollable {
   Default(ColorPalette),
   Secondary(ColorPalette),
}

impl scrollable::StyleSheet for CustomScrollable {
   fn active(&self) -> scrollable::Scrollbar {
      scrollable::Scrollbar {
         background: Some(Background::Color(match self {
            Self::Default(palette) => palette.base.background,
            Self::Secondary(palette) => palette.base.foreground,
         })),
         scroller: scrollable::Scroller {
            color: match self {
               Self::Default(palette) => palette.base.foreground,
               Self::Secondary(palette) => palette.base.background,
            },
            border_radius: 2.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
         },
         border_radius: 0.0,
         border_width: 0.0,
         border_color: Color::TRANSPARENT,
      }
   }

   fn hovered(&self) -> scrollable::Scrollbar {
      let active = self.active();
      scrollable::Scrollbar {
         scroller: scrollable::Scroller { ..active.scroller },
         ..active
      }
   }

   fn dragging(&self) -> scrollable::Scrollbar {
      let hoverd = self.hovered();
      scrollable::Scrollbar {
         scroller: scrollable::Scroller { ..hoverd.scroller },
         ..hoverd
      }
   }
}

pub enum CustomPicklist {
   Default(ColorPalette),
   Secondary(ColorPalette),
}

impl pick_list::StyleSheet for CustomPicklist {
   fn menu(&self) -> pick_list::Menu {
      pick_list::Menu {
         text_color: match self {
            Self::Default(palette) | Self::Secondary(palette) => palette.bright.surface,
         },
         background: match self {
            Self::Default(palette) | Self::Secondary(palette) => {
               Background::Color(palette.base.background)
            }
         },
         border_width: 1.0,
         border_color: match self {
            Self::Default(palette) => palette.base.background,
            Self::Secondary(palette) => palette.base.foreground,
         },
         selected_background: Background::Color(match self {
            Self::Default(palette) | Self::Secondary(palette) => Color {
               a: 0.15,
               ..palette.normal.primary
            },
         }),
         selected_text_color: match self {
            Self::Default(palette) | Self::Secondary(palette) => palette.bright.primary,
         },
      }
   }

   fn active(&self) -> pick_list::Style {
      pick_list::Style {
         text_color: match self {
            Self::Default(palette) | Self::Secondary(palette) => palette.bright.surface,
         },
         background: match self {
            Self::Default(palette) => palette.base.background.into(),
            Self::Secondary(palette) => palette.base.foreground.into(),
         },
         border_width: match self {
            Self::Default(_) => 1.0,
            Self::Secondary(_) => 0.0,
         },
         border_color: match self {
            Self::Default(palette) => palette.base.foreground,
            Self::Secondary(palette) => palette.base.background,
         },
         border_radius: 2.0,
         icon_size: 0.5,
      }
   }

   fn hovered(&self) -> pick_list::Style {
      let active = self.active();

      pick_list::Style {
         text_color: match self {
            Self::Default(palette) | Self::Secondary(palette) => palette.bright.primary,
         },
         background: match self {
            Self::Default(_) => active.background,
            Self::Secondary(palette) => Background::Color(Color {
               a: 0.15,
               ..palette.normal.primary
            }),
         },
         ..active
      }
   }
}

pub enum CustomCheckbox {
   Default(ColorPalette),
   AlwaysChecked(ColorPalette),
}

impl checkbox::StyleSheet for CustomCheckbox {
   fn active(&self, is_checked: bool) -> checkbox::Style {
      checkbox::Style {
         checkmark_color: match self {
            Self::Default(palette) | Self::AlwaysChecked(palette) => palette.base.foreground,
         },
         background: Background::Color(match self {
            Self::Default(palette) | Self::AlwaysChecked(palette) => {
               if is_checked {
                  palette.bright.primary
               } else {
                  palette.base.background
               }
            }
         }),
         border_color: match self {
            Self::Default(palette) | Self::AlwaysChecked(palette) => palette.bright.primary,
         },
         border_radius: 3.0,
         border_width: 2.0,
      }
   }

   fn hovered(&self, is_checked: bool) -> checkbox::Style {
      self.active(is_checked)
   }
}

pub enum CustomTextInput {
   Default(ColorPalette),
   Secondary(ColorPalette),
   Error(ColorPalette),
}
impl text_input::StyleSheet for CustomTextInput {
   fn active(&self) -> text_input::Style {
      text_input::Style {
         background: Background::Color(match self {
            Self::Default(palette) | Self::Error(palette) => palette.base.foreground,
            Self::Secondary(palette) => palette.base.background,
         }),
         border_radius: 12.0,
         border_width: 0.0,
         border_color: match self {
            Self::Default(palette) | Self::Error(palette) => palette.base.foreground,
            Self::Secondary(palette) => palette.base.background,
         },
      }
   }

   fn focused(&self) -> text_input::Style {
      text_input::Style {
         border_width: 1.0,
         border_color: match self {
            Self::Default(palette) => Color {
               a: 0.5,
               ..palette.normal.primary
            },
            Self::Secondary(palette) => Color {
               a: 0.5,
               ..palette.normal.secondary
            },
            Self::Error(palette) => Color {
               a: 0.5,
               ..palette.normal.error
            },
         },
         ..self.active()
      }
   }

   fn placeholder_color(&self) -> Color {
      match self {
         Self::Default(palette) | Self::Secondary(palette) | Self::Error(palette) => {
            palette.normal.surface
         }
      }
   }

   fn value_color(&self) -> Color {
      match self {
         Self::Default(palette) | Self::Secondary(palette) | Self::Error(palette) => {
            palette.bright.primary
         }
      }
   }

   fn selection_color(&self) -> Color {
      match self {
         Self::Default(palette) | Self::Secondary(palette) | Self::Error(palette) => {
            palette.bright.secondary
         }
      }
   }

   fn hovered(&self) -> text_input::Style {
      self.focused()
   }
}