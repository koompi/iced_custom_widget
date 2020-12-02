use super::super::{
   pref::{Pref, Category},
   main::SystemMessage
};
use crate::components::grid::Grid;
use crate::styles::custom_styles::CustomContainer;
use crate::utils::themes::Theme;
use iced::{
   Container, Column, Row, Space, Length, Text, Align, scrollable, Scrollable
};