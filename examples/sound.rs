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
use styles::{
    ButtonStyle, ContainerStyle, PickListStyle, SliderStyle,
};

use std::collections::HashMap;
// use rodio::Source;
use std::fmt;
use std::hash::Hash;
// use std::fs::File;
// use std::io::BufReader;

use std::ops::Index;
use std::path::PathBuf;
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
    TestSoundEffect(usize),
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
    hash_sounds: HashMap<SoundEffectType, PathBuf>,
    effect_type: SoundEffectType,
    volume: u32,
    speed: u32,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SoundEffectType {
    Bootup,
    ShutDown,
    Logout,
    Wakeup,
    VolumnUpDown,
    // Notification,
    // LowBattery,
    // SendIconLauncher,
    // EmptyTrash,
    // Plugin,
    // Plugout,
    // RemoveDevConnected,
    // RemovableDevRemoved,
    // ErrorSound,
}
impl SoundEffectType {
    const ALL: [SoundEffectType; 5] = [
        SoundEffectType::Bootup,
        SoundEffectType::ShutDown,
        SoundEffectType::Logout,
        SoundEffectType::Wakeup,
        SoundEffectType::VolumnUpDown,
        // SoundEffectType::Notification,
        // SoundEffectType::LowBattery,
        // SoundEffectType::SendIconLauncher,
        // SoundEffectType::EmptyTrash,
        // SoundEffectType::Plugin,
        // SoundEffectType::Plugout,
        // SoundEffectType::RemoveDevConnected,
        // SoundEffectType::RemovableDevRemoved,
        // SoundEffectType::RemovableDevRemoved,
    ];
}
impl fmt::Display for SoundEffectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SoundEffectType::Bootup => "Bootup",
                SoundEffectType::ShutDown => "Shutdown",
                SoundEffectType::Logout => "Log out",
                SoundEffectType::Wakeup => "Wake Up",
                SoundEffectType::VolumnUpDown => "Volume +/-",
                // SoundEffectType::Notification => "Notifications",
                // SoundEffectType::LowBattery => "Low battery",
                // SoundEffectType::SendIconLauncher => "Send icon in Launcher to Desktop",
                // SoundEffectType::EmptyTrash => "Empty Trash",
                // SoundEffectType::Plugin => "Plug in",
                // SoundEffectType::Plugout => "Plug out",
                // SoundEffectType::RemoveDevConnected => "Removable device connected",
                // SoundEffectType::RemovableDevRemoved => "Removable device removed",
                // SoundEffectType::ErrorSound => "Error",
            }
        )
    }
}
impl Default for SoundEffectType {
    fn default() -> Self {
        SoundEffectType::Bootup
    }
}
impl SettingsSoundEffect {
    pub fn new() -> Self {
        Self {
            file: dirs_next::config_dir().unwrap(),
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

// impl Index<String> for SettingsSoundEffect {
//     type Output = PathBuf;
//     fn index(&self, key: String) -> &PathBuf {
//         &self.hash_sounds.index(&key)
//     }
// }
impl Index<&SoundEffectType> for SettingsSoundEffect {
    type Output = PathBuf;
    fn index(&self, key: &SoundEffectType) -> &Self::Output {
        &self.hash_sounds.index(&key)
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
        let str_con = |f: &str| -> String { f.to_string() };
        let mut vec_sounds: Vec<String> = vec![
            str_con("Bootup"),
            str_con("Shutdown"),
            str_con("Log out"),
            str_con("Wake Up"),
            str_con("Volume +/-"),
            // str_con("Notifications"),
            // str_con("Low battery"),
            // str_con("Send icon in Launcher to Desktop"),
            // str_con("Empty Trash"),
            // str_con("Plug in"),
            // str_con("Plug out"),
            // str_con("Removeable device connected"),
            // str_con("Removable device removed"),
            // str_con("Error"),
        ];
        let mut vec_tuple: Vec<(button::State, button::State, String)> = Vec::new();
        vec_sounds.iter_mut().for_each(|name| {
            vec_tuple.push((button::State::new(), button::State::new(), name.clone()))
        });
        let mut sound_effect_hash: HashMap<SoundEffectType, PathBuf> = HashMap::new();
        match playback::read_directory(
            std::path::PathBuf::new()
                .join(standart_path::sys_data_dir().unwrap().join("syssettings")),
        ) {
            Ok(mut path) =>
            {
                #[allow(const_item_mutation)]
                for (i, j) in SoundEffectType::ALL[..].iter_mut().zip(path.iter_mut()) {
                    sound_effect_hash.insert(*i, j.to_path_buf());
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        for (i, j) in &sound_effect_hash {
            println!("key: {} value: {:?}", i, j);
        }
        println!(
            "Booup value: {:?}",
            sound_effect_hash.index(&SoundEffectType::Bootup)
        );
        (
            Self {
                FONT_SIZE: 12,
                sound_effecs: SettingsSoundEffect {
                    hash_sounds: sound_effect_hash,
                    ..Default::default()
                },
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
                SoundBackEnd::initialize();
                Command::none()
            }
            SoundMessage::InputLevelChanged(val) => {
                self.input_level = val;
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
            SoundMessage::TestSoundEffect(idx) => {
                let key = SoundEffectType::ALL[idx];
                let value = self.sound_effecs.hash_sounds.index(&key);
                match playback::run(&value) {
                    Ok(()) => println!("sucesss"),
                    Err(e) => println!("Error: {}", e),
                }
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
                                        self.out_value,
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
            .push(Container::new(Text::new("Input").size(self.FONT_SIZE + 12)))
            .spacing(10)
            .push(
                Container::new(
                    Row::new()
                        .width(Length::Fill)
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
                Container::new(
                    Row::new()
                        .align_items(Align::Center)
                        .spacing(10)
                        .push(Text::new("Sound Effects"))
                        .push(Space::with_width(Length::Fill))
                        .push(Toggler::new(
                            self.is_sound_effect,
                            String::from(""),
                            SoundMessage::SoundEffect,
                        )),
                )
                .padding(10)
                .style(ContainerStyle::LightGrayCircle),
            )
            .push(if self.is_sound_effect {
                self.sample_effects.iter_mut().enumerate().fold(
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
                                    .on_press(SoundMessage::TestSoundEffect(idx)),
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
                )
            } else {
                Column::new()
            });
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
            size: (800, 800),
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

mod playback {

    use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
    use std::fs::read_dir;
    use std::path::PathBuf;
    use std::time::Duration;
    // NOTE: You probably want to investigate the
    // mixer feature for real use cases.
    struct Sound {
        data: Vec<u8>,
        volume: f32,
        pos: usize,
    }

    pub fn read_directory(in_path: std::path::PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut list_sounds: Vec<PathBuf> = Vec::new();
        let sound_dir = "sounds";
        if in_path.join(sound_dir).exists() {
            for path in read_dir(in_path.join(sound_dir))? {
                let dir = path?;
                list_sounds.push(dir.path());
            }
        } else {
            make_dir(&in_path, sound_dir)?;
            let paths = read_dir(in_path)?;
            paths.for_each(|val| {
                println!("Name: {:?}", val);
            });
        }
        Ok(list_sounds)
    }
    pub fn make_dir(in_path: &std::path::PathBuf, name: &str) -> Result<bool, std::io::Error> {
        std::fs::create_dir(in_path.join(name))?;
        Ok(true)
    }

    impl AudioCallback for Sound {
        type Channel = u8;

        fn callback(&mut self, out: &mut [u8]) {
            for dst in out.iter_mut() {
                // With channel type u8 the "silence" value is 128 (middle of the 0-2^8 range) so we need
                // to both fill in the silence and scale the wav data accordingly. Filling the silence
                // once the wav is finished is trivial, applying the volume is more tricky. We need to:
                // * Change the range of the values from [0, 255] to [-128, 127] so we can multiply
                // * Apply the volume by multiplying, this gives us range [-128*volume, 127*volume]
                // * Move the resulting range to a range centered around the value 128, the final range
                //   is [128 - 128*volume, 128 + 127*volume] – scaled and correctly positioned
                //
                // Using value 0 instead of 128 would result in clicking. Scaling by simply multiplying
                // would not give correct results.
                let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
                let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
                let scaled = (scaled_signed_float + 128.0) as u8;
                *dst = scaled;
                self.pos += 1;
            }
        }
    }
    pub fn run(path: &std::path::PathBuf) -> Result<(), String> {
        let sdl_context = sdl2::init().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None,     // default
        };
        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                let wav = AudioSpecWAV::load_wav(path).expect("Could not load test WAV file");
                let cvt = AudioCVT::new(
                    wav.format,
                    wav.channels,
                    wav.freq,
                    spec.format,
                    spec.channels,
                    spec.freq,
                )
                .expect("Could not convert WAV file");
                let data = cvt.convert(wav.buffer().to_vec());
                // initialize the audio callback
                Sound {
                    data: data,
                    volume: 0.50,
                    pos: 0,
                }
            })
            .unwrap();
        // Start playback
        device.resume();
        // std::thread::spawn(|| {
        // Play for a second
        std::thread::sleep(Duration::from_millis(1_000));
        // });

        // Device is automatically closed when dropped

        Ok(())
    }
}

mod standart_path {
    use std::path::PathBuf;
    pub fn sys_data_dir() -> Option<PathBuf> {
        Some(PathBuf::new().join("/usr/share/"))
    }
}

mod SoundBackEnd {

    pub fn initialize() {}
    pub fn volume_up(level: u32) {}
    pub fn volumn_down(level: u32) {}
    pub fn mute_sound(is_mute: bool) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn sound_test() {
        println!("{}", Volume::MAX);
        assert_eq!(2 + 2, 3);
    }
}

mod styles {
use iced::{button, container, Color, Background, Vector};
use iced::slider::{self, Handle, HandleShape};
use iced::pick_list::{self, Menu};
pub enum ButtonStyle {
    Default,
    Circular(u8, u8, u8, f32),
    BigCircular(u8, u8, u8, f32),
    CircleRadius(u8, u8, u8, f32, f32, Color),
    Transparent,
 }
 
 impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
       button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: match self {
                ButtonStyle::Default => Some(Background::Color([0.87, 0.87, 0.87].into())),
                ButtonStyle::Circular(c1, c2, c3, p)
                | ButtonStyle::CircleRadius(c1, c2, c3, p, _, _)
                | ButtonStyle::BigCircular(c1, c2, c3, p) => {
                    Some(Background::Color(Color::from_rgba8(*c1, *c2, *c3, *p)))
                }
                ButtonStyle::Transparent => Some(Background::Color(Color::TRANSPARENT)),
            },
            border_radius: match self {
                ButtonStyle::Default | ButtonStyle::Circular(_, _, _, _) => 4.0,
                ButtonStyle::BigCircular(_, _, _, _) => 25.0,
                ButtonStyle::Transparent => 0.0,
                ButtonStyle::CircleRadius(_, _, _, _, r, _) => *r,
            },
            border_width: 0.0,
            border_color: [0.7, 0.7, 0.7].into(),
            text_color: match self {
                ButtonStyle::Default
                | ButtonStyle::BigCircular(_, _, _, _)
                | ButtonStyle::Circular(_, _, _, _) => Color::WHITE,
                ButtonStyle::Transparent => Color::BLACK,
                ButtonStyle::CircleRadius(_, _, _, _, _, color) => *color,
            },
        }
    }
 }
 
 pub enum ContainerStyle {
    Custom,
    InkColor,
    LightGray,
    White,
    LightGrayCircle,
    Black,
 }
 impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> container::Style {
       container::Style {
            text_color: None,
            background: match self {
                ContainerStyle::Custom => {
                    Some(Background::Color(Color::from_rgba8(223, 228, 234, 1.0)))
                }
                ContainerStyle::InkColor => {
                    Some(Background::from(Color::from_rgba8(206, 214, 224, 1.0)))
                }
                ContainerStyle::LightGray => {
                    Some(Background::from(Color::from_rgba8(215, 219, 221, 1.0)))
                }
                ContainerStyle::White => {
                    Some(Background::from(Color::from_rgba8(255, 255, 255, 1.0)))
                }
                ContainerStyle::LightGrayCircle => {
                    Some(Background::from(Color::from_rgba8(215, 219, 221, 0.5)))
                }
                ContainerStyle::Black => Some(Background::from(Color::BLACK)),
            },
            border_radius: match self {
                ContainerStyle::Custom
                | ContainerStyle::LightGrayCircle
                | ContainerStyle::White
                | ContainerStyle::InkColor
                | ContainerStyle::Black => 10.0,
                ContainerStyle::LightGray => 0.0,
            },
            border_width: 0.0,
            border_color: Color::from_rgba8(255, 255, 255, 1.0),
        }
    }
 }
pub enum SliderStyle {
    Default,
    Circle,
}

impl slider::StyleSheet for SliderStyle {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (
                Color::from_rgba8(128, 139, 150, 1.0),
                Color::from_rgba8(128, 139, 150, 1.0),
            ),
            handle: Handle {
                shape: match self {
                    SliderStyle::Default => HandleShape::Rectangle {
                        width: 24,
                        border_radius: 8.0,
                    },
                    SliderStyle::Circle => HandleShape::Circle { radius: 12.0 },
                },
                color: Color::from_rgba8(128, 139, 150, 1.5),
                border_color: Color::from_rgba8(44, 62, 80, 1.0),
                border_width: 1.0,
            },
        }
    }
    fn hovered(&self) -> slider::Style {
        let active = self.active();
        slider::Style {
            handle: Handle {
                color: Color::from_rgba8(205, 213, 203, 1.0),
                ..active.handle
            },
            ..active
        }
    }
    fn dragging(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: Handle {
                color: Color::from_rgba8(205, 213, 203, 1.0),
                ..active.handle
            },
            ..active
        }
    }
}
pub struct PickListStyle;

impl pick_list::StyleSheet for PickListStyle {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgba8(215, 219, 221, 1.0)),
            border_width: 0.5,
            border_color: [0.7, 0.7, 0.7].into(),
            selected_text_color: Color::WHITE,
            selected_background: Background::Color(Color::from_rgba8(86, 101, 115, 1.0)),
        }
    }
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgba8(215, 219, 221, 0.5)),
            border_radius: 10.0,
            border_width: 0.0,
            border_color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            border_color: Color::BLACK,
            ..self.active()
        }
    }
}

}