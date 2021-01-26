use super::table_column::{self, ColumnKey, TableColumn};
use iced_graphics::{Backend, Primitive};
use iced_native::{
    event::{self, Event},
    layout::{self, Limits, Node},
    mouse, text, Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct TableHeader<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    spacing: u16,
    leeway: u16,
    columns: Vec<Element<'a, Message, Renderer>>,
    names: Vec<String>,
    on_resize: Option<(u16, Box<dyn Fn(ResizeEvent) -> Message + 'a>)>,
}

impl<'a, Message, Renderer> TableHeader<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    pub fn new<Key: ColumnKey>(
        state: &'a mut State,
        children: Vec<(String, TableColumn<'a, Message, Renderer, Key>)>,
    ) -> Self {
        let mut columns = Vec::with_capacity(children.len());
        let mut names = Vec::with_capacity(children.len());
        for (name, column) in children {
            names.push(name);
            columns.push(column.into());
        }

        Self {
            state,
            spacing: 1,
            width: Length::Shrink,
            height: Length::Shrink,
            leeway: 0,
            columns,
            names,
            on_resize: None,
        }
    }

    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn on_resize<F>(mut self, leeway: u16, f: F) -> Self
    where
        F: 'a + Fn(ResizeEvent) -> Message,
    {
        self.leeway = leeway;
        self.on_resize = Some((leeway, Box::new(f)));
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TableHeader<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        layout::flex::resolve(
            layout::flex::Axis::Horizontal,
            renderer,
            &limits,
            0.0,
            self.spacing as f32,
            Align::Start,
            &self.columns,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            defaults,
            &self.columns,
            layout,
            cursor_position,
            viewport,
            self.state.resize_hovering,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.spacing.hash(state);
        self.leeway.hash(state);

        for child in &self.columns {
            child.hash_layout(state);
        }
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let in_bounds = layout.bounds().contains(cursor_position);
        let mut event_status = event::Status::Ignored;

        if self.state.resizing || in_bounds {
            let dividers = self
                .columns
                .iter()
                .enumerate()
                .zip(layout.children())
                .filter_map(|((idx, _), layout)| {
                    Some((idx, layout.position().x + layout.bounds().width))
                })
                .collect::<Vec<_>>();
            if self.on_resize.is_some() {
                event_status = event::Status::Captured;
                if !self.state.resizing {
                    self.state.resize_hovering = false;
                }
                for (idx, divider) in dividers.iter() {
                    if cursor_position.x > (divider - self.leeway as f32)
                        && cursor_position.x < (divider + self.leeway as f32)
                    {
                        if !self.state.resize_hovering {
                            self.state.resizing_idx = *idx;
                        }
                        self.state.resize_hovering = true;
                    }
                }
            }

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if self.state.resize_hovering {
                        self.state.resizing = true;
                        self.state.starting_cursor_pos = Some(cursor_position);
                        self.state.starting_left_width = layout
                            .children()
                            .nth(self.state.resizing_idx)
                            .unwrap()
                            .bounds()
                            .width;
                        self.state.starting_right_width = layout
                            .children()
                            .nth(self.state.resizing_idx + 1)
                            .unwrap()
                            .bounds()
                            .width;
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if self.state.resizing {
                        self.state.resizing = false;
                        self.state.starting_cursor_pos.take();
                        if let Some((_, on_resize)) = &self.on_resize {
                            messages.push(on_resize(ResizeEvent::Finished));
                        }
                    }
                }
                Event::Mouse(mouse::Event::CursorMoved { x, .. }) => {
                    if self.state.resizing {
                        let delta = x - self.state.starting_cursor_pos.unwrap().x;

                        let left_width = self.state.starting_left_width;
                        let right_width = self.state.starting_right_width;

                        let max_width = left_width + right_width - 30.0;

                        let left_width = (left_width + delta).max(30.0).min(max_width) as u16;
                        let left_name = self.names[self.state.resizing_idx].as_str();
                        let right_width = (right_width - delta).max(30.0).min(max_width) as u16;
                        let right_name = self.names[self.state.resizing_idx].as_str();
                        if let Some((_, on_resize)) = &self.on_resize {
                            messages.push(on_resize(ResizeEvent::ResizeColumn {
                                left_name: String::from(left_name),
                                left_width,
                                right_name: String::from(right_name),
                                right_width,
                            }));
                        }
                    }
                }
                _ => {
                    event_status = event::Status::Ignored;
                }
            }
        } else {
            self.state.resize_hovering = false;
        }

        self.columns
            .iter_mut()
            .zip(layout.children())
            .map(|(child, layout)| {
                child.on_event(
                    event.clone(),
                    layout,
                    cursor_position,
                    messages,
                    renderer,
                    clipboard,
                )
            })
            .fold(event_status, event::Status::merge)
    }
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub resize_hovering: bool,
    pub resizing: bool,
    pub resizing_idx: usize,
    pub starting_cursor_pos: Option<Point>,
    pub starting_left_width: f32,
    pub starting_right_width: f32,
}

#[derive(Debug, Clone)]
pub enum ResizeEvent {
    ResizeColumn {
        left_name: String,
        left_width: u16,
        right_name: String,
        right_width: u16,
    },
    Finished,
}

pub trait Renderer: table_column::Renderer {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        resize_hovering: bool,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        resize_hovering: bool,
    ) -> Self::Output {
        let mut mouse_interaction = if resize_hovering {
            mouse::Interaction::ResizingHorizontally
        } else {
            mouse::Interaction::default()
        };

        (
            Primitive::Group {
                primitives: content
                    .iter()
                    .zip(layout.children())
                    .map(|(child, layout)| {
                        let (primitive, new_mouse_interaction) =
                            child.draw(self, defaults, layout, cursor_position, viewport);

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse_interaction,
        )
    }
}

impl<'a, Message, Renderer> From<TableHeader<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    fn from(header: TableHeader<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(header)
    }
}
