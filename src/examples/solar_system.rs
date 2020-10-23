// use iced::{
//     canvas::{self, Drawable, Frame},
//     executor, futures, window, Application, Canvas, Color, Command, Container, Element, Length,
//     Point, Size, Subscription, Vector,
// };
// use iced_futures::subscription::Recipe;
// use std::time::{Duration, Instant};

// #[derive(Debug)]
// pub struct State {
//     pub start: Instant,
//     pub current: Instant,
//     pub stars: Vec<(Point, f32)>,
// }

// pub struct SolarSystem {
//     pub state: State,
//     pub solar_system: canvas::layer::Cache<State>,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Tick(Instant),
// }

// impl State {
//     const SUN_RADIUS: f32 = 70.0;
//     const ORBIT_RADIUS: f32 = 150.0;
//     const EARTH_RADIUS: f32 = 12.0;
//     const MOON_RADIUS: f32 = 4.0;
//     const MOON_DISTANCE: f32 = 28.0;

//     pub fn new() -> Self {
//         let now = Instant::now();
//         let (width, height) = window::Settings::default().size;

//         Self {
//             start: now,
//             current: now,
//             stars: {
//                 use rand::Rng;

//                 let mut rng = rand::thread_rng();

//                 (0..=100)
//                     .map(|_| {
//                         (
//                             Point::new(
//                                 rng.gen_range(0.0, width as f32),
//                                 rng.gen_range(0.0, height as f32),
//                             ),
//                             rng.gen_range(0.5, 2.),
//                         )
//                     })
//                     .collect()
//             },
//         }
//     }

//     pub fn update(&mut self, now: Instant) {
//         self.current = now;
//     }
// }

// impl Drawable for State {
//     fn draw(&self, frame: &mut Frame) {
//         use canvas::{Path, Stroke};
//         use std::f32::consts::PI;

//         let center = frame.center();

//         let space = Path::rectangle(Point::new(0.0, 0.0), frame.size());

//         let stars = Path::new(|path| {
//             for (p, size) in &self.stars {
//                 path.rectangle(*p, Size::new(*size, *size));
//             }
//         });

//         let sun = Path::circle(center, Self::SUN_RADIUS);
//         let orbit = Path::circle(center, Self::ORBIT_RADIUS);

//         frame.fill(&space, Color::BLACK);
//         frame.fill(&stars, Color::WHITE);
//         frame.fill(&sun, Color::from_rgb8(0xF9, 0xD7, 0x1C));
//         frame.stroke(
//             &orbit,
//             Stroke {
//                 width: 1.0,
//                 color: Color::from_rgba8(0, 153, 255, 0.1),
//                 ..Stroke::default()
//             },
//         );

//         let elapsed = self.current - self.start;
//         let elapsed_seconds = elapsed.as_secs() as f32;
//         let elapsed_millis = elapsed.subsec_millis() as f32;

//         frame.with_save(|frame| {
//             frame.translate(Vector::new(center.x, center.y));
//             frame.rotate(
//                 (2.0 * PI / 60.0) * elapsed_seconds + (2.0 * PI / 60_000.0) * elapsed_millis,
//             );
//             frame.translate(Vector::new(Self::ORBIT_RADIUS, 0.0));

//             let earth = Path::circle(Point::ORIGIN, Self::EARTH_RADIUS);
//             let shadow = Path::rectangle(
//                 Point::new(0.0, -Self::EARTH_RADIUS),
//                 Size::new(Self::EARTH_RADIUS * 4.0, Self::EARTH_RADIUS * 2.0),
//             );

//             frame.fill(&earth, Color::from_rgb8(0x6B, 0x93, 0xD6));

//             frame.with_save(|frame| {
//                 frame.rotate(
//                     ((2.0 * PI) / 6.0) * elapsed_seconds + ((2.0 * PI) / 6_000.0) * elapsed_millis,
//                 );
//                 frame.translate(Vector::new(0.0, Self::MOON_DISTANCE));

//                 let moon = Path::circle(Point::ORIGIN, Self::MOON_RADIUS);
//                 frame.fill(&moon, Color::WHITE);
//             });

//             frame.fill(
//                 &shadow,
//                 Color {
//                     a: 0.7,
//                     ..Color::BLACK
//                 },
//             );
//         });
//     }
// }

// pub fn every(duration: Duration) -> Subscription<Instant> {
//     Subscription::from_recipe(Every(duration))
// }

// pub struct Every(Duration);

// impl<H, I> Recipe<H, I> for Every
// where
//     H: std::hash::Hasher,
// {
//     type Output = Instant;

//     fn hash(&self, state: &mut H) {
//         use std::hash::Hash;

//         std::any::TypeId::of::<Self>().hash(state);
//         self.0.hash(state);
//     }

//     fn stream(
//         self: Box<Self>,
//         _input: futures::stream::BoxStream<'static, I>,
//     ) -> futures::stream::BoxStream<'static, Self::Output> {
//         use futures::stream::StreamExt;
//         async_std::stream::interval(self.0)
//             .map(|_| Instant::now())
//             .boxed()
//     }
// }

// impl Application for SolarSystem {
//     type Executor = executor::Default;
//     type Message = Message;
//     type Flags = ();

//     fn new(_flags: ()) -> (Self, Command<Message>) {
//         (
//             SolarSystem {
//                 state: State::new(),
//                 solar_system: Default::default(),
//             },
//             Command::none(),
//         )
//     }

//     fn title(&self) -> String {
//         String::from("Solar system - Iced")
//     }

//     fn update(&mut self, message: Message) -> Command<Message> {
//         match message {
//             Message::Tick(instant) => {
//                 self.state.update(instant);
//                 self.solar_system.clear();
//             }
//         }

//         Command::none()
//     }

//     fn subscription(&self) -> Subscription<Message> {
//         every(std::time::Duration::from_millis(10)).map(|instant| Message::Tick(instant))
//     }

//     fn view(&mut self) -> Element<Message> {
//         let canvas = Canvas::new()
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .push(self.solar_system.with(&self.state));

//         Container::new(canvas)
//             .width(Length::Fill)
//             .height(Length::Fill)
//             .center_x()
//             .center_y()
//             .into()
//     }
// }
