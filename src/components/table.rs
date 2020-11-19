use super::{
    table_column::{self, ColumnKey},
    table_header::{self, TableHeader},
    table_row::{self, TableRow},
};
use iced_graphics::{Backend, Primitive};
use iced_native::{
    event::{self, Event},
    layout::{self, flex::Axis, Limits, Node},
    mouse, text, Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

pub struct Table<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    // state: &'a mut State<Key>,
    width: Length,
    height: Length,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Table<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + text::Renderer,
{
    pub fn new<Key: ColumnKey>(header: TableHeader<'a, Message, Renderer, Key>) -> Self {
        Self::rows(header, Vec::new())
    }

    pub fn rows<Key: ColumnKey>(
        header: TableHeader<'a, Message, Renderer, Key>,
        rows: Vec<TableRow<'a, Message, Renderer>>,
    ) -> Self {
        let mut children = Vec::new();
        children.push(header.into());
        for row in rows {
            children.push(row.into());
        }

        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            children,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Table<'a, Message, Renderer>
where
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
        // self.rows.insert(0, self.header);

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
        self.children.iter().for_each(|e| e.hash_layout(state));
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
            .fold(event::Status::Ignored, event::Status::merge)
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
        let mut mouse_interaction = mouse::Interaction::default();

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
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer> From<Table<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: Clone + 'static,
{
    fn from(table: Table<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(table)
    }
}
