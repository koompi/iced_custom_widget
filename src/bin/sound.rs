use iced::{
    button, executor, pick_list, scrollable, slider, text_input, window, Align, Application,
    Button, Column, Command, Container, Element, HorizontalAlignment, Length, PickList, Row, Rule,
    Scrollable, Settings, Slider, Space, Text, TextInput, VerticalAlignment,
};
use iced_custom_widget as icw;
use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::{
    buttons::ButtonStyle, containers::ContainerStyle, pick_list::PickListStyle, rules::RuleStyle,
    text_input::InputStyle,
};
use std::fmt;
/// # use iced_native::{renderer::Null, Element, Grid as NativeGrid, Text};
#[derive(Default, Debug, Clone)]
pub struct Sound {
    choice: Choice,
    scroll_content: scrollable::State,
    pick_out_dev: pick_list::State<OutputDevice>,
    selected_out_dev: OutputDevice,
    is_boost_sound: bool,
    is_muted: bool,
    out_value: f32,
    balance_val: f32,
    slider_output: slider::State,
    balace_state: slider::State,
    mute_out_sound: button::State,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
    C,
}
impl Default for Choice {
    fn default() -> Self {
        Choice::A
    }
}

#[derive(Debug, Clone)]
pub enum SoundMessage {
    TabSelect(Choice),
    SeletedOut(OutputDevice),
    EnableBoostSound(bool),
    SoundOutChanged(f32),
    MutedSound,
    MuteSound,
    BalanceChanged(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputDevice {
    Internal,
    External,
}
impl Default for OutputDevice {
    fn default() -> Self {
        OutputDevice::Internal
    }
}
impl OutputDevice {
    const ALL: [OutputDevice; 2] = [OutputDevice::Internal, OutputDevice::External];
}
impl fmt::Display for OutputDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OutputDevice::Internal => "Internal (HDA Intel PCH)",
                OutputDevice::External => "External",
            }
        )
    }
}
impl Application for Sound {
    type Executor = executor::Default;
    type Flags = ();
    type Message = SoundMessage;

    fn title(&self) -> String {
        String::from("GridView Applicaiton ")
    }
    fn new(_flags: ()) -> (Sound, Command<SoundMessage>) {
        (
            Self {
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            SoundMessage::TabSelect(tab) => {
                self.choice = tab;
                Command::none()
            }
            SoundMessage::SeletedOut(out_dev) => {
                self.selected_out_dev = out_dev;
                Command::none()
            }
            SoundMessage::EnableBoostSound(is_enable) => {
                self.is_boost_sound = is_enable;
                Command::none()
            }
            SoundMessage::SoundOutChanged(val) => {
                self.out_value = val;
                Command::none()
            }
            SoundMessage::BalanceChanged(val) => {
                self.balance_val = val;
                Command::none()
            }
            SoundMessage::MuteSound => Command::none(),
            SoundMessage::MutedSound => {
                self.is_muted = !self.is_muted;
                Command::none()
            }
            _ => Command::none(),
        }
    }
    fn view(&mut self) -> Element<Self::Message> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(10)
            .push(
                Tab::new(
                    Choice::A,
                    Some(self.choice),
                    SoundMessage::TabSelect,
                    tab_content('\u{f028}', "Output"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::B,
                    Some(self.choice),
                    SoundMessage::TabSelect,
                    tab_content('\u{f130}', "Input"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            )
            .push(
                Tab::new(
                    Choice::C,
                    Some(self.choice),
                    SoundMessage::TabSelect,
                    tab_content('\u{f5fd}', "Sound Effects"),
                )
                .width(Length::Fill)
                .height(Length::Units(50)),
            );
        let output_content = Column::new()
            .spacing(10)
            .push(Text::new("Output"))
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Output Device").size(24))
                        .push(
                            PickList::new(
                                &mut self.pick_out_dev,
                                &OutputDevice::ALL[..],
                                Some(self.selected_out_dev),
                                SoundMessage::SeletedOut,
                            )
                            .style(PickListStyle {})
                            .width(Length::Fill),
                        ),
                )
                .width(Length::Fill)
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(
                            Row::new()
                                .push(Text::new("Output Volume"))
                                .push(Space::with_width(Length::Fill))
                                .push(Text::new(&format!("{}%", self.out_value.to_string()))),
                        )
                        .push(
                            Row::new()
                                .align_items(Align::Center)
                                .spacing(4)
                                .push(
                                    Button::new(
                                        &mut self.mute_out_sound,
                                        Icon::new(if self.is_muted {
                                            '\u{f026}'
                                        } else {
                                            '\u{f028}'
                                        }),
                                    )
                                    .on_press(SoundMessage::MutedSound)
                                    .style(ButtonStyle::Transparent),
                                )
                                .push(
                                    Slider::new(
                                        &mut self.slider_output,
                                        0.0..=100.0,
                                        self.out_value,
                                        SoundMessage::SoundOutChanged,
                                    )
                                    .step(1.0)
                                    .width(Length::Fill),
                                )
                                .push(Icon::new('\u{f027}')),
                        ),
                )
                .width(Length::Fill)
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Volume Boost"))
                        .push(Space::with_width(Length::Fill))
                        .push(Toggler::new(
                            self.is_boost_sound,
                            String::from(""),
                            SoundMessage::EnableBoostSound,
                        )),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            ).push(if self.is_boost_sound {
                Container::new(Text::new("If the volume is lounder than 100%, it may distort audio and be harmdul to your speaker")).padding(10)
            }else {
                Container::new(Space::with_height(Length::Units(0)))
            })
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(Text::new("Left/Right Balance"))
                        .push(
                            Slider::new(
                                &mut self.balace_state,
                                0.0..=100.0,
                                self.balance_val,
                                SoundMessage::BalanceChanged,
                            )
                            .step(1.0),
                        )
                        .push(
                            Row::new()
                                .push(Text::new("Left"))
                                .push(Space::with_width(Length::Fill))
                                .push(Text::new("Right")),
                        ), // .push(),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            );
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(20)
            .push(match self.choice {
                Choice::A => Container::new(output_content),
                Choice::B => Container::new(Text::new("B Content")),
                Choice::C => Container::new(Text::new("C Content")),
            });
        let netsidebar_scroll = Scrollable::new(&mut self.scroll_content)
            .push(row)
            .padding(10)
            .scrollbar_width(4)
            .scroller_width(4);
        let whole_content = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Container::new(netsidebar_scroll.height(Length::Fill))
                    .style(ContainerStyle::White)
                    .width(Length::FillPortion(4))
                    .height(Length::Fill),
            )
            .push(Rule::vertical(10))
            .push(
                Container::new(contnet.height(Length::Fill))
                    .width(Length::FillPortion(9))
                    .height(Length::Fill)
                    .style(ContainerStyle::White), // .padding(10),
            );
        let container = Container::new(whole_content)
            .width(Length::Fill)
            .center_x()
            .center_y();
        Container::new(container)
            .style(ContainerStyle::LightGray)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
fn tab_content(unicode: char, name: &str) -> Row<'static, SoundMessage> {
    Row::new()
        .push(Icon::new(unicode).size(24))
        .push(Text::new(name).size(16))
        .align_items(Align::Center)
        .spacing(8)
}
pub fn init() -> iced::Result {
    Sound::run(Settings {
        window: window::Settings {
            size: (400, 400),
            min_size: Some((600, 800)),
            max_size: Some((800, 800)),
            ..Default::default()
        },
        ..Default::default()
    })
}

fn main() {
    match init() {
        Ok(()) => println!("run success"),
        Err(e) => eprintln!("error: {:?}", e),
    }
}
