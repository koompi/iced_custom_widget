use super::table_cell::{self, TableCell};
use crate::styles::table_row::StyleSheet;
use iced_graphics::{Backend, Primitive};
use iced_native::{
    event::{self, Event},
    layout::{self, Limits, Node},
    mouse, Align, Background, Clipboard, Color, Element, Hasher, Layout, Length, Point, Rectangle,
    Widget,
};

pub struct TableRow<'a, Message, Renderer: self::Renderer> {
    cells: Vec<Element<'a, Message, Renderer>>,
    width: Length,
    max_width: u32,
    min_width: u32,
    has_background: bool,
    is_pressed: bool,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> TableRow<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + self::Renderer + table_cell::Renderer,
{
    pub fn new(has_background: bool) -> Self {
        Self::cells(has_background, Vec::new())
    }

    pub fn cells(has_background: bool, cells: Vec<TableCell<'a, Message, Renderer>>) -> Self {
        let cells: Vec<Element<_, _>> = cells.into_iter().map(|cell| cell.into()).collect();
        Self {
            cells,
            width: Length::Shrink,
            max_width: u32::MAX,
            min_width: 0,
            has_background,
            is_pressed: false,
            style: Renderer::Style::default(),
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TableRow<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .width(self.width)
            .height(self.height())
            .max_width(self.max_width)
            .min_width(self.min_width);

        layout::flex::resolve(
            layout::flex::Axis::Horizontal,
            renderer,
            &limits,
            0.0,
            1.,
            Align::Start,
            &self.cells,
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mouse_over = layout.bounds().contains(cursor_position);

        if mouse_over {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    self.is_pressed = true;
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    self.is_pressed = false;
                }
                _ => {}
            }
            event::Status::Captured
        } else {
            event::Status::Ignored
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
        renderer.draw(
            defaults,
            layout,
            cursor_position,
            viewport,
            self.has_background,
            self.is_pressed,
            &self.cells,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.min_width.hash(state);
        self.max_width.hash(state);
        self.width.hash(state);
        self.cells.iter().for_each(|cell| cell.hash_layout(state));
    }
}

pub trait Renderer: iced_native::Renderer {
    type Style: Default;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        has_background: bool,
        is_pressed: bool,
        cells: &[Element<'_, Message, Self>],
        style: &Self::Style,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        has_background: bool,
        is_pressed: bool,
        cells: &[Element<'_, Message, Self>],
        style: &Self::Style,
    ) -> Self::Output {
        let bounds = layout.bounds();
        let mouse_over = bounds.contains(cursor_position);

        let styling = if is_pressed {
            style.pressed()
        } else if has_background {
            style.selected()
        } else {
            style.active()
        };
        let background = Primitive::Quad {
            bounds,
            background: styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: styling.border_radius as f32,
            border_width: 0 as f32,
            border_color: Color::TRANSPARENT,
        };
        let table_row = Primitive::Group {
            primitives: cells
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| {
                    let (primitive, _) =
                        child.draw(self, defaults, layout, cursor_position, viewport);

                    primitive
                })
                .collect(),
        };
        (
            Primitive::Group {
                primitives: vec![background, table_row],
            },
            if mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer> From<TableRow<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + self::Renderer,
{
    fn from(table_row: TableRow<'a, Message, Renderer>) -> Self {
        Self::new(table_row)
    }
}
