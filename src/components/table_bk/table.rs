use super::{
    table_column::{self, ColumnKey},
    table_header::{self, TableHeader},
    table_row::{self, TableRow},
};
use iced_graphics::{Backend, Primitive};
use iced_native::{
    event::{self, Event}, scrollable,
    layout::{self, flex::Axis, Limits, Node}, Size,
    mouse, text, Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct Table<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Renderer: self::Renderer,
{
    state: &'a mut State<Key>,
    width: Length,
    height: Length,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer, Key> Table<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + text::Renderer,
{
    pub fn new(state: &'a mut State<Key>, header: TableHeader<'a, Message, Renderer>) -> Self {
        Self {
            state,
            width: Length::Shrink,
            height: Length::Shrink,
            children: vec![header.into()],
        }
    }

    pub fn push(mut self, row: TableRow<'a, Message, Renderer>) -> Self {
        self.children.push(row.into());
        self
    }
}

impl<'a, Message, Renderer, Key> Widget<Message, Renderer> for Table<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.width).height(self.height);

        if self.children.is_empty() {
            return Node::new(Size::ZERO);
        } else {
            layout::flex::resolve(
                Axis::Vertical,
                renderer,
                &limits,
                0.,
                0.,
                Align::Center,
                &self.children,
            )
        }
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
            layout,
            cursor_position,
            viewport,
            &self.children,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.children.iter().for_each(|c| c.hash_layout(state));
    }

    fn on_event(&mut self, event: Event, layout: Layout<'_>, cursor_position: Point, messages: &mut Vec<Message>, renderer: &Renderer, clipboard: Option<&dyn Clipboard>) -> event::Status {
        let event_status = event::Status::Ignored;

        if !self.children.is_empty() {
            self.children
                .iter_mut()
                .zip(layout.children())
                .map(|(children, layout)| {
                    children.on_event(
                        event.clone(),
                        layout,
                        cursor_position,
                        messages,
                        renderer,
                        clipboard,
                    )
                })
                .fold(event_status, event::Status::merge);
        }
        event_status
    }
}

#[derive(Debug, Default, Clone)]
pub struct State<Key: ColumnKey> {
    pub scrollable_state: scrollable::State,
    pub column_key: Option<Key>,
    pub sort_direction: Option<SortDirection>,
    pub columns: Vec<table_column::State<Key>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn toggle(self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

pub trait Renderer: table_row::Renderer + table_header::Renderer {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        children: &[Element<'_, Message, Self>],
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        children: &[Element<'_, Message, Self>],
    ) -> Self::Output {
        let layout_bound = layout.bounds();
        let is_mouse_over = layout_bound.contains(cursor_position);
        let mut mouse_interaction = if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        let content = Primitive::Group {
            primitives: children
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
        };
        (
            content,
            mouse_interaction
        )
    }
}

impl<'a, Message, Renderer, Key> From<Table<'a, Message, Renderer, Key>> for Element<'a, Message, Renderer>
where
    Key: ColumnKey,
    Renderer: 'a + self::Renderer,
    Message: Clone + 'static,
{
    fn from(table: Table<'a, Message, Renderer, Key>) -> Element<'a, Message, Renderer> {
        Element::new(table)
    }
}
