use iced_graphics::{Backend, Primitive};
use iced_native::{
    button,
    event::{self, Event},
    layout::{Limits, Node},
    mouse, row, text, Background, Clipboard, Color, Element, Hasher, HorizontalAlignment, Layout,
    Length, Point, Rectangle, Row, Text, VerticalAlignment, Widget,
};
use std::hash::Hash;

pub struct TableColumn<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Renderer: self::Renderer + text::Renderer,
{
    state: &'a mut State<Key>,
    padding: u16,
    numeric: bool,
    on_pressed: Option<Message>,
    style: Renderer::Style,
    font: Renderer::Font,
}

impl<'a, Message, Renderer, Key> TableColumn<'a, Message, Renderer, Key>
where
    Renderer: self::Renderer + text::Renderer,
    Key: ColumnKey,
{
    pub fn new(state: &'a mut State<Key>) -> Self {
        Self {
            state,
            numeric: false,
            padding: 5,
            on_pressed: None,
            style: Renderer::Style::default(),
            font: Renderer::Font::default(),
        }
    }

    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_pressed = Some(on_press);
        self
    }

    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }
}

impl<'a, Message, Renderer, Key> Widget<Message, Renderer>
    for TableColumn<'a, Message, Renderer, Key>
where
    Key: ColumnKey,
    Message: Clone,
    Renderer: self::Renderer + row::Renderer + text::Renderer,
{
    fn width(&self) -> Length {
        self.state.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits.width(self.state.width).height(self.height());
        Row::<(), Renderer>::new()
            .padding(self.padding)
            .push(Text::new(&self.state.key.title()).width(Length::Fill))
            .layout(renderer, &limits)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        let bounds = layout.bounds();
        let text_layout = layout.children().next().unwrap();
        let text = text::Renderer::draw(
            renderer,
            defaults,
            text_layout.bounds(),
            &self.state.key.title(),
            renderer.default_size(),
            self.font,
            None,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
        );
        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            defaults,
            bounds,
            self.on_pressed.is_none(),
            self.state.is_pressed,
            is_mouse_over,
            text,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.state.width.hash(state);
        self.state.key.hash(state);
        self.padding.hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.on_pressed.is_some() {
                    let bounds = layout.bounds();
                    self.state.is_pressed = bounds.contains(cursor_position);
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(on_pressed) = self.on_pressed.clone() {
                    let bounds = layout.bounds();
                    let is_clicked = self.state.is_pressed && bounds.contains(cursor_position);
                    self.state.is_pressed = false;

                    if is_clicked {
                        messages.push(on_pressed);
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }
        event::Status::Ignored
    }
}

#[derive(Debug, Clone)]
pub struct State<Key>
where
    Key: ColumnKey,
{
    pub key: Key,
    pub is_pressed: bool,
    pub width: Length,
}

impl<Key> Default for State<Key>
where
    Key: ColumnKey,
{
    fn default() -> Self {
        Self {
            key: Default::default(),
            is_pressed: false,
            width: Length::Shrink,
        }
    }
}

pub trait ColumnKey
where
    Self: Default + Clone + Copy + PartialEq + Hash + Eq,
{
    fn title(self) -> String;
    fn as_string(self) -> String;
    fn all() -> Vec<Self>;
}

pub trait Renderer: button::Renderer + row::Renderer {
    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        is_disabled: bool,
        is_pressed: bool,
        is_mouse_over: bool,
        text: Self::Output,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    fn draw(
        &mut self,
        _defaults: &Self::Defaults,
        bounds: Rectangle,
        is_disabled: bool,
        is_pressed: bool,
        is_mouse_over: bool,
        (text, _): Self::Output,
        style: &Self::Style,
    ) -> Self::Output {
        let style = if is_disabled {
            style.disabled()
        } else if is_pressed {
            style.pressed()
        } else if is_mouse_over {
            style.hovered()
        } else {
            style.active()
        };

        let background = Primitive::Quad {
            bounds,
            background: style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        };
        (
            Primitive::Group {
                primitives: vec![background, text],
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer, Key> From<TableColumn<'a, Message, Renderer, Key>>
    for Element<'a, Message, Renderer>
where
    Key: 'a + ColumnKey,
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    fn from(
        table_column: TableColumn<'a, Message, Renderer, Key>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(table_column)
    }
}
