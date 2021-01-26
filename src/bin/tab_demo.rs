use iced_custom_widget as icw;
use icw::components::Tab;
use icw::components::Toggler;
use iced::{
    executor, window, Align, Application, Checkbox, Color, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Svg, Text, VerticalAlignment,
};
/// # use iced_native::{renderer::Null, Element, Grid as NativeGrid, Text};
fn main() {
    init();
}
#[derive(Debug)]
pub struct TabContent {
    choice: Choice,
    content1: ContentUI,
}
impl Default for TabContent {
    fn default() -> Self {
        Self {
            choice: Choice::A,
            content1: ContentUI::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    A,
    B,
}
#[derive(Debug, Copy, Clone)]
pub enum AppMessage {
    TabSelect(Choice),
    Conmsg(Conmsg),
}

impl Application for TabContent {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMessage;

    fn title(&self) -> String {
        String::from("GridView Applicaiton ")
    }
    fn new(_flags: ()) -> (TabContent, Command<AppMessage>) {
        (TabContent::default(), Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::TabSelect(select) => {
                self.choice = select;
                Command::none()
            }
            AppMessage::Conmsg(msg) => {
                self.content1.update(msg);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let row = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .push(
                Tab::new(
                    Choice::A,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    Text::new("Hello")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Units(100))
                .height(Length::Units(100)),
            )
            .push(
                Tab::new(
                    Choice::B,
                    Some(self.choice),
                    AppMessage::TabSelect,
                    Text::new("B")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                )
                .width(Length::Units(100))
                .height(Length::Units(100)),
            );
        let contnet = Column::new()
            .height(Length::Fill)
            .align_items(Align::Center)
            .push(match self.choice {
                Choice::A => self.content1.view().map(move |msg| AppMessage::Conmsg(msg)),
                Choice::B => Text::new("Content 2").into(),
            });
        let whole_content: Element<_> = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(row.width(Length::FillPortion(2)).height(Length::Fill))
            .push(contnet.width(Length::FillPortion(9)).height(Length::Fill))
            .into();
        let container = Container::new(whole_content)
            .width(Length::Fill)
            .center_x()
            .center_y();
        Container::new(container)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn init() {
    TabContent::run(Settings::default());
}
#[derive(Default, Debug)]
pub struct ContentState {
    is_active: bool,
}
#[derive(Default, Debug)]
pub struct ContentUI {
    constate: ContentState,
}

impl ContentUI {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, msg: Conmsg) {
        match msg {
            Conmsg::ToggleChange(value) => {
                self.constate.is_active = value;
            }
        }
    }
    pub fn view(&mut self) -> Element<Conmsg> {
        let ContentState { is_active } = self.constate;
        Column::new()
            .push(
                Toggler::new(is_active, String::from("Active"), Conmsg::ToggleChange)
                    .width(Length::Shrink),
            )
            .into()
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Conmsg {
    ToggleChange(bool),
}