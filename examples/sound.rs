use iced::{
    button, executor, pick_list, scrollable, slider, window, Align, Application, Button, Column,
    Command, Container, Element, Length, PickList, Row, Rule, Scrollable, Settings, Slider, Space,
    Subscription, Text,
};

use iced_custom_widget as icw;
use iced_native::window::Event;
use iced_native::Event::Window;
use icw::components::Icon;
use icw::components::Tab;
use icw::components::Toggler;
use icw::styles::{
    buttons::ButtonStyle, containers::ContainerStyle, pick_list::PickListStyle, slider::SliderStyle,
};
// use rodio::Source;
use std::fmt;
// use std::fs::File;
// use std::io::BufReader;
use std::path::PathBuf;
use PlaySound::run;
#[allow(non_snake_case)]
#[derive(Default, Debug, Clone)]
pub struct Sound {
    choice: Choice,
    scroll_content: scrollable::State,
    pick_out_dev: pick_list::State<OutputDevice>,
    pick_in_dev: pick_list::State<InputDevice>,
    selected_in_dev: InputDevice,
    selected_out_dev: OutputDevice,
    is_boost_sound: bool,
    is_muted: bool,
    enable_sound_effect: bool,
    is_sound_effect: bool,
    is_auto_noise_suppression: bool,
    is_in_muted: bool,
    sound_effecs: SettingsSoundEffect,
    sample_effects: Vec<(button::State, button::State, String)>,
    out_value: f32,
    effect_tick: usize,
    balance_val: f32,
    input_level: f32,
    input_val: f32,
    slider_output: slider::State,
    slider_input_level: slider::State,
    slider_input: slider::State,
    balace_state: slider::State,
    mute_out_sound: button::State,
    mute_in_sound: button::State,
    sound_effect_state: button::State,

    window_size: (u32, u32),
    FONT_SIZE: u16,
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
    SeletedIn(InputDevice),
    EnableBoostSound(bool),
    SoundOutChanged(f32),
    SoundInChanged(f32),
    SoundEffect(bool),
    InputLevelChanged(f32),
    AutomatedSoundSuppression(bool),
    MutedSound,
    MuteSound,
    MutedInSound,
    EnableEffect(usize),
    TestSoundEffect,
    BalanceChanged(f32),
    WindowResize((u32, u32)),
    CloseApp,
    Escape,
}
pub type SoundEffectErorr = Result<bool, std::io::Error>;

pub trait SoundEffect {
    fn play(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn stop(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn pause(&mut self, file: PathBuf) -> SoundEffectErorr;
    fn speed(&self) -> u32;
    fn volume(&self) -> u32;
}
#[derive(Debug, Default, Clone)]
pub struct SettingsSoundEffect {
    file: std::path::PathBuf,
    effect_type: SoundEffectType,
    volume: u32,
    speed: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum SoundEffectType {
    Bootup,
    ShutDown,
    Logout,
    Wakeup,
    VolumnUpDown,
    Notification,
    LowBattery,
    SendIconLauncher,
    EmptyTrash,
    Plugin,
    Plugout,
    RemoveDevConnected,
    RemovableDevRemoved,
    ErrorSound,
}
impl Default for SoundEffectType {
    fn default() -> Self {
        SoundEffectType::Bootup
    }
}
impl SettingsSoundEffect {
    pub fn new() -> Self {
        Self {
            file: dirs::config_dir().unwrap(),
            ..Default::default()
        }
    }
}
impl SoundEffect for SettingsSoundEffect {
    fn play(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn pause(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn stop(&mut self, file: PathBuf) -> SoundEffectErorr {
        unimplemented!("Function is unimplemented");
    }
    fn speed(&self) -> u32 {
        unimplemented!("Function is unimplemented");
    }
    fn volume(&self) -> u32 {
        unimplemented!("Function is unimplemented");
    }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDevice {
    Internal,
    External,
}
impl Default for InputDevice {
    fn default() -> Self {
        InputDevice::Internal
    }
}
impl InputDevice {
    const ALL: [InputDevice; 2] = [InputDevice::Internal, InputDevice::External];
}
impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputDevice::Internal => "Internal (HDA Intel PCH)",
                InputDevice::External => "External",
            }
        )
    }
}

// impl Sound {
//     fn repsonsive_font(&mut self, w: u32) {
//         match w {
//             500..=599 => {
//                 if self.FONT_SIZE <= 8 {
//                     self.FONT_SIZE = 8;
//                 } else {
//                     self.FONT_SIZE -= 4;
//                 }
//             }
//             600..=665 => {
//                 if self.FONT_SIZE <= 8 {
//                     self.FONT_SIZE = 8;
//                 } else {
//                     self.FONT_SIZE -= 4;
//                 }
//             }
//             666..=1920 => {
//                 if self.FONT_SIZE >= 18 {
//                     self.FONT_SIZE = 12;
//                 } else {
//                     self.FONT_SIZE += 1;
//                 }
//             }
//             _ => {}
//         }
//     }
// }

impl Application for Sound {
    type Executor = executor::Default;
    type Flags = ();
    type Message = SoundMessage;

    fn title(&self) -> String {
        String::from("GridView Applicaiton ")
    }
    fn new(_flags: ()) -> (Sound, Command<SoundMessage>) {
        let str_con = |f: &str| -> String { f.to_string() };
        let mut vec_sounds: Vec<String> = vec![
            str_con("Bootup"),
            str_con("Shutdown"),
            str_con("Log out"),
            str_con("Wake Up"),
            str_con("Volume +/-"),
            str_con("Notifications"),
            str_con("Low battery"),
            str_con("Send icon in Launcher to Desktop"),
            str_con("Empty Trash"),
            str_con("Plug in"),
            str_con("Plug out"),
            str_con("Removeable device connected"),
            str_con("Removable device removed"),
            str_con("Error"),
        ];
        let mut vec_tuple: Vec<(button::State, button::State, String)> = Vec::new();
        vec_sounds.iter_mut().for_each(|name| {
            vec_tuple.push((button::State::new(), button::State::new(), name.clone()))
        });
        (
            Self {
                FONT_SIZE: 12,
                sound_effecs: SettingsSoundEffect::new(),
                enable_sound_effect: true,
                sample_effects: vec_tuple,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> Subscription<SoundMessage> {
        iced_native::subscription::events_with(|event, status| {
            if let iced_native::event::Status::Captured = status {
                return None;
            }

            match event {
                Window(Event::Resized { width, height }) => {
                    // println!("width: {}", wi`dth);
                    Some(SoundMessage::WindowResize((width, height)))
                }
                iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    modifiers,
                    key_code,
                }) => match key_code {
                    iced::keyboard::KeyCode::W => {
                        if modifiers.control {
                            Some(SoundMessage::CloseApp)
                        } else {
                            None
                        }
                    }
                    iced::keyboard::KeyCode::Escape => Some(SoundMessage::Escape),
                    _ => None,
                },
                _ => None,
            }
        })
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
            SoundMessage::SoundEffect(is_effectd) => {
                self.is_sound_effect = is_effectd;
                Command::none()
            }
            SoundMessage::SoundOutChanged(val) => {
                self.out_value = val;
                Command::none()
            }
            SoundMessage::WindowResize((w, h)) => {
                self.window_size.0 = w;
                self.window_size.1 = h;
                // self.repsonsive_font(w);
                Command::none()
            }
            SoundMessage::SoundInChanged(val) => {
                self.input_val = val;
                Command::none()
            }
            SoundMessage::CloseApp => {
                println!("Application clsoe with Ctrl+ W");
                Command::none()
            }
            SoundMessage::Escape => {
                println!("Escape key press:");
                Command::none()
            }
            SoundMessage::BalanceChanged(val) => {
                self.balance_val = val;
                Command::none()
            }
            SoundMessage::EnableEffect(idx) => {
                self.effect_tick = idx;
                PlaySound::run();
                Command::none()
            }
            SoundMessage::AutomatedSoundSuppression(is_auto) => {
                self.is_auto_noise_suppression = is_auto;
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
        let effect_enable = self.enable_sound_effect;
        let current_tick = self.effect_tick;
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
            .push(Text::new("Output").size(self.FONT_SIZE + 12))
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Output Device").size(self.FONT_SIZE + 10))
                        .push(
                            PickList::new(
                                &mut self.pick_out_dev,
                                &OutputDevice::ALL[..],
                                Some(self.selected_out_dev),
                                SoundMessage::SeletedOut,
                            ).text_size(self.FONT_SIZE + 2)
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
                                .push(Text::new("Output Volume").size(self.FONT_SIZE + 10))
                                .push(Space::with_width(Length::Fill))
                                .push(Text::new(&format!("{}%", self.out_value.to_string())).size(self.FONT_SIZE + 10)),
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
                                        self.input_val,
                                        SoundMessage::SoundOutChanged,
                                    ).style(SliderStyle::Circle)
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
                        .push(Text::new("Volume Boost").size(self.FONT_SIZE + 10))
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
                Container::new(Text::new("If the volume is lounder than 100%, it may distort audio and be harmdul to your speaker").size(self.FONT_SIZE + 8)).padding(10)
            }else {
                Container::new(Space::with_height(Length::Units(0)))
            })
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(Text::new("Left/Right Balance").size(self.FONT_SIZE + 10))
                        .push(
                            Slider::new(
                                &mut self.balace_state,
                                0.0..=100.0,
                                self.balance_val,
                                SoundMessage::BalanceChanged,
                            ).style(SliderStyle::Default)
                            .step(1.0),
                        )
                        .push(
                            Row::new()
                                .push(Text::new("Left").size(self.FONT_SIZE + 8))
                                .push(Space::with_width(Length::Fill))
                                .push(Text::new("Right").size(self.FONT_SIZE + 8)),
                        ), // .push(),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            );
        let input_content = Column::new()
            .push(Container::new(Text::new("Input").size(self.FONT_SIZE + 12)).padding(10))
            .spacing(10)
            .push(
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Input Devices"))
                        .push(
                            PickList::new(
                                &mut self.pick_in_dev,
                                &InputDevice::ALL[..],
                                Some(self.selected_in_dev),
                                SoundMessage::SeletedIn,
                            )
                            .style(PickListStyle {}),
                        ),
                )
                .style(ContainerStyle::LightGrayCircle)
                .padding(10),
            )
            .push(
                Container::new(
                    Column::new().spacing(10).push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(4)
                            .push(Button::new(
                                &mut self.mute_in_sound,
                                Icon::new(if self.is_in_muted {
                                    '\u{f026}'
                                } else {
                                    '\u{f028}'
                                }),
                            ))
                            .push(
                                Slider::new(
                                    &mut self.slider_input,
                                    0.0..=100.0,
                                    self.input_val,
                                    SoundMessage::SoundInChanged,
                                )
                                .style(SliderStyle::Default)
                                .step(1.0),
                            )
                            .push(Icon::new('\u{f028}')),
                    ),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(10)
                        .push(Text::new("Input Level"))
                        .push(
                            Row::new()
                                .push(Icon::new('\u{f192}'))
                                .spacing(10)
                                .push(
                                    Slider::new(
                                        &mut self.slider_input_level,
                                        0.0..=100.0,
                                        self.input_level,
                                        SoundMessage::InputLevelChanged,
                                    )
                                    .step(1.0)
                                    .style(SliderStyle::Default),
                                )
                                .push(Icon::new('\u{f141}')),
                        ),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(
                Container::new(Toggler::new(
                    self.is_auto_noise_suppression,
                    String::from("Automatic Noise Suppression"),
                    SoundMessage::AutomatedSoundSuppression,
                ))
                .style(ContainerStyle::LightGrayCircle)
                .padding(10),
            );
        let sound_effects = Column::new()
            .spacing(10)
            .push(
                Row::new()
                    .spacing(10)
                    .push(Text::new("Sound Effects"))
                    .push(Space::with_width(Length::Fill))
                    .push(Toggler::new(
                        self.is_sound_effect,
                        String::from(""),
                        SoundMessage::SoundEffect,
                    )),
            )
            .push(self.sample_effects.iter_mut().enumerate().fold(
                Column::new().spacing(10).align_items(Align::Center),
                |col_sound, (idx, (enable_state, state, name))| {
                    col_sound.push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(10)
                            .width(Length::Fill)
                            .push(
                                Button::new(
                                    enable_state,
                                    Row::new().push(Text::new(name.as_str())),
                                )
                                .width(Length::Fill)
                                .style(ButtonStyle::Transparent)
                                .on_press(SoundMessage::TestSoundEffect),
                            )
                            // .push(Space::with_width(Length::Fill))
                            .push(
                                Button::new(
                                    state,
                                    Icon::new(if effect_enable && current_tick == idx {
                                        '\u{f058}'
                                    } else {
                                        '\u{f111}'
                                    }),
                                )
                                .padding(4)
                                .style(ButtonStyle::Transparent)
                                .on_press(SoundMessage::EnableEffect(idx)),
                            ),
                    )
                },
            ));
        // f058 tick-circle
        // f111 circle
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(20)
            .push(match self.choice {
                Choice::A => Container::new(output_content),
                Choice::B => Container::new(input_content),
                Choice::C => Container::new(sound_effects),
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
fn tab_content<'l>(unicode: char, name: &str) -> Row<'l, SoundMessage> {
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
            min_size: Some((300, 800)),
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

mod PlaySound {

    use sdl2::audio::{self, AudioCallback, AudioDevice, AudioSpecDesired, AudioSpecWAV};
    use sdl2::Sdl;
    use std::time::Duration;

    pub struct CopiedData {
        bytes: Vec<u8>,
        position: usize,
    }
    impl AudioCallback for CopiedData {
        type Channel = u8;
        fn callback(&mut self, data: &mut [u8]) {
            let (start, end) = (self.position, self.position + data.len());
            self.position += data.len();
            let audio_data = &self.bytes[start..end];
            for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
                *dst = *src;
            }
        }
    }
    struct WrappedData {
        audio: AudioSpecWAV,
        position: usize,
    }
    impl AudioCallback for WrappedData {
        type Channel = u8;
        fn callback(&mut self, data: &mut [u8]) {
            let (start, end) = (self.position, self.position + data.len());
            self.position += data.len();
            let audio_data = &self.audio.buffer()[start..end];
            for (src, dst) in audio_data.iter().zip(data.iter_mut()) {
                *dst = *src;
            }
        }
    }
    struct SquareWave {
        phase_inc: f32,
        phase: f32,
        volume: f32,
    }

    impl AudioCallback for SquareWave {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            // Generate a square wave
            for x in out.iter_mut() {
                *x = if self.phase <= 0.8 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }

    unsafe impl Send for WrappedData {}
    pub fn run() {
        let handler = std::thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let audio_subsystem = sdl_context.audio().unwrap();
            let desired_spec = AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1), // mono
                samples: None,     // default sample size
            };
            let device = audio_subsystem
                .open_playback(None, &desired_spec, |spec| {
                    // initialize the audio callback
                    SquareWave {
                        phase_inc: 440.0 / spec.freq as f32,
                        phase: 0.0,
                        volume: 0.25,
                    }
                })
                .unwrap();
            // Start playback
            device.resume();
            // Play for 2 seconds
            // std::thread::sleep(Duration::from_millis(1000));
        });
        match handler.join() {
            Ok(()) => println!("run success"),
            Err(e) => println!("thread crashed: {:?} ", e),
        }
    }
}
