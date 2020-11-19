use crate::components::table_column::{self, ColumnKey, TableColumn};
use iced_graphics::{Backend, Primitive};
use iced_native::{
    event::{self, Event},
    layout::{self, Limits, Node},
    mouse, text, Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct TableHeader<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Renderer: self::Renderer,
{
    state: &'a mut State<Key>,
    width: Length,
    height: Length,
    spacing: u16,
    leeway: u16,
    children: Vec<Element<'a, Message, Renderer>>,
    on_resize: Option<(u16, Box<dyn Fn(ResizeEvent) -> Message + 'a>)>,
}

impl<'a, Message, Renderer, Key> TableHeader<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    pub fn new(state: &'a mut State<Key>) -> Self {
        let children = vec![];

        // for column in state.columns.iter_mut() {
        //     children.push(TableColumn::new(column).into());
        // }

        Self {
            state,
            spacing: 1,
            width: Length::Shrink,
            height: Length::Shrink,
            leeway: 0,
            children,
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

    fn trigger_resize(
        &self,
        left_name: String,
        left_width: u16,
        right_name: String,
        right_width: u16,
        messages: &mut Vec<Message>,
    ) {
        if let Some((_, on_column_resize)) = &self.on_resize {
            messages.push(on_column_resize(ResizeEvent::ResizeColumn {
                left_name,
                left_width,
                right_name,
                right_width,
            }));
        }
    }

    fn trigger_finished(&self, messages: &mut Vec<Message>) {
        if let Some((_, on_column_resize)) = &self.on_resize {
            messages.push(on_column_resize(ResizeEvent::Finished));
        }
    }
}

impl<'a, Message, Renderer, Key> Widget<Message, Renderer>
    for TableHeader<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
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
            &self.children,
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
            &self.children,
            layout,
            cursor_position,
            viewport,
            self.state.internal.resize_hovering,
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

        for child in &self.children {
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

        if self.state.internal.resizing || in_bounds {
            let dividers = self
                .children
                .iter()
                .enumerate()
                .zip(layout.children())
                .filter_map(|((idx, _), layout)| {
                    Some((idx, layout.position().x + layout.bounds().width))
                })
                .collect::<Vec<_>>();

            if self.on_resize.is_some() {
                event_status = event::Status::Captured;
                if !self.state.internal.resizing {
                    self.state.internal.resize_hovering = false;
                }

                for (idx, divider) in dividers.iter() {
                    if cursor_position.x > (divider - self.leeway as f32)
                        && cursor_position.x < (divider + self.leeway as f32)
                    {
                        if !self.state.internal.resize_hovering {
                            self.state.internal.resizing_idx = *idx;
                        }

                        self.state.internal.resize_hovering = true;
                    }
                }
            }

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    if self.state.internal.resize_hovering {
                        self.state.internal.resizing = true;
                        self.state.internal.starting_cursor_pos = Some(cursor_position);
                        self.state.internal.starting_left_width = layout
                            .children()
                            .nth(self.state.internal.resizing_idx)
                            .unwrap()
                            .bounds()
                            .width;
                        self.state.internal.starting_right_width = layout
                            .children()
                            .nth(self.state.internal.resizing_idx + 1)
                            .unwrap()
                            .bounds()
                            .width;
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    if self.state.internal.resizing {
                        self.state.internal.resizing = false;
                        self.state.internal.starting_cursor_pos.take();
                        self.trigger_finished(messages);
                    }
                }
                Event::Mouse(mouse::Event::CursorMoved { x, .. }) => {
                    if self.state.internal.resizing {
                        let delta = x - self.state.internal.starting_cursor_pos.unwrap().x;

                        let left_width = self.state.internal.starting_left_width;
                        let right_width = self.state.internal.starting_right_width;

                        let max_width = left_width + right_width - 30.0;

                        let left_width = (left_width + delta).max(30.0).min(max_width) as u16;
                        let left_name = &self.state.columns[self.state.internal.resizing_idx]
                            .key
                            .as_string();
                        let right_width = (right_width - delta).max(30.0).min(max_width) as u16;
                        let right_name = &self.state.columns[self.state.internal.resizing_idx]
                            .key
                            .as_string();

                        self.trigger_resize(
                            left_name.clone(),
                            left_width,
                            right_name.clone(),
                            right_width,
                            messages,
                        );
                    }
                }
                _ => {
                    event_status = event::Status::Ignored;
                }
            }
        } else {
            self.state.internal.resize_hovering = false;
        }

        self.children
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
pub struct State<Key: ColumnKey> {
    pub internal: InternalState,
    pub previous_column_key: Option<Key>,
    pub previous_sort_direction: Option<SortDirection>,
    pub columns: Vec<table_column::State<Key>>,
}

#[derive(Debug, Default, Clone)]
pub struct InternalState {
    resize_hovering: bool,
    resizing: bool,
    resizing_idx: usize,
    starting_cursor_pos: Option<Point>,
    starting_left_width: f32,
    starting_right_width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

// impl SortDirection {
//     fn toggle(self) -> Self {
//         match self {
//             Self::Asc => Self::Desc,
//             Self::Desc => Self::Asc,
//         }
//     }
// }

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

impl<'a, Message, Renderer, Key> From<TableHeader<'a, Message, Renderer, Key>>
    for Element<'a, Message, Renderer>
where
    Key: 'a + ColumnKey,
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    fn from(header: TableHeader<'a, Message, Renderer, Key>) -> Element<'a, Message, Renderer> {
        Element::new(header)
    }
}
