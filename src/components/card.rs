use iced_native::{
    layout::{self, flex::Axis, Limits, Node}, Clipboard, Column, column,
    mouse, Align, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget, Color, Background, Vector
};
use iced_graphics::{
    defaults, Defaults, Primitive, Backend, backend
};
use crate::styles::card::StyleSheet;

pub struct Card<'a, Message, Renderer: self::Renderer>
{
    state: &'a mut State,
    header: Element<'a, Message, Renderer>,
    body: Element<'a, Message, Renderer>,
    footer: Element<'a, Message, Renderer>,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    min_width: u32,
    min_height: u32,
    padding: u16,
    margin: u16,
    on_pressed: Option<Message>,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> Card<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer + column::Renderer,
{
    pub fn new<E>(state: &'a mut State, header: E, body: E, footer: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        Self {
            state,
            header: header.into(),
            body: body.into(),
            footer: footer.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: 0,
            max_height: 0,
            min_width: 0,
            min_height: 0,
            padding: Renderer::DEFAULT_PADDING,
            margin: 0,
            on_pressed: None,
            style: Renderer::Style::default(),
        }
    }

    pub fn body<E>(mut self, body: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.body = body.into();
        self
    }

    pub fn header<E>(mut self, header: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.header = header.into();
        self
    }

    pub fn footer<E>(mut self, footer: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.footer = footer.into();
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

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
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

    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }

    pub fn on_pressed(mut self, msg: Message) -> Self {
        self.on_pressed = Some(msg);
        self
    }

    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Card<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer + column::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = f32::from(self.padding);
        let limits = limits
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .max_height(self.max_height)
            .pad(padding);

        // Column::<Message, Renderer>::new()
        //     .width(self.width)
        //     .spacing(self.padding)
        //     .align_items(Align::Center)
        //     .push(self.header)
        //     .push(self.body)
        //     .push(self.footer)
        //     .layout(renderer, &limits)

        // let mut content = column.layout(renderer, &limits);
        // content.move_to(Point::new(padding, padding));
        // let size = limits.resolve(content.size()).pad(padding);
        // layout::Node::with_children(size, vec![content])
        let header = self.header.layout(&renderer, &limits);
        let mut body = self.body.layout(&renderer, &limits);
        body.move_to(Point::new(padding, padding));
        let footer = self.footer.layout(&renderer, &limits);
        let size = limits.resolve(body.size()).pad(self.padding as f32);
        Node::with_children(size, vec![header, body, footer])
        // layout::flex::resolve(
        //     Axis::Vertical,
        //     renderer,
        //     &limits,
        //     padding,
        //     20.,
        //     Align::Center,
        //     &[self.header, self.body, self.footer],
        // )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        self.min_height.hash(state);
        self.min_width.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);
        self.header.hash_layout(state);
        self.body.hash_layout(state);
        self.footer.hash_layout(state);
        self.padding.hash(state);
        self.margin.hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.on_pressed.is_some() {
                    let bounds = layout.bounds();

                    self.state.is_pressed = bounds.contains(cursor_position);
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(on_pressed) = self.on_pressed.clone() {
                    let bounds = layout.bounds();

                    let is_clicked = self.state.is_pressed && bounds.contains(cursor_position);

                    self.state.is_pressed = false;

                    if is_clicked {
                        messages.push(on_pressed);
                    }
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        // let column = column::Renderer::draw(
        //     renderer,
        //     defaults,
        //     &[self.header, self.body, self.footer],
        //     layout,
        //     cursor_position
        // );

        self::Renderer::draw(
            renderer,
            defaults,
            cursor_position,
            self.on_pressed.is_none(),
            self.state.is_pressed,
            layout.bounds(),
            &self.header,
            &self.body,
            &self.footer,
            layout.children().next().unwrap(),
            &self.style,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_pressed: bool,
}

impl State {
    pub fn new() -> State {
        State::default()
    }
}

pub trait Renderer: iced_native::Renderer {
    type Style: std::default::Default;

    const DEFAULT_PADDING: u16;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        bounds: Rectangle,
        header: &Element<'_, Message, Self>,
        body: &Element<'_, Message, Self>,
        footer: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_PADDING: u16 = 8;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        bounds: Rectangle,
        header: &Element<'_, Message, Self>,
        body: &Element<'_, Message, Self>,
        footer: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
        style: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let styling = if is_disabled {
            style.disabled()
        } else if is_mouse_over {
            if is_pressed {
                style.pressed()
            } else {
                style.hovered()
            }
        } else {
            style.active()
        };

        let (header, _) = header.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: styling.text_color,
                },
            },
            content_layout,
            cursor_position,
        );

        let (body, _) = body.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: styling.text_color,
                },
            },
            content_layout,
            cursor_position,
        );

        let (footer, _) = footer.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: styling.text_color,
                },
            },
            content_layout,
            cursor_position,
        );
        let background = Primitive::Quad {
            bounds,
            background: styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: styling.border_radius,
            border_width: styling.border_width,
            border_color: styling.border_color,
        };

        let shadow = Primitive::Quad {
            bounds: Rectangle {
                x: bounds.x + styling.shadow_offset.x,
                y: bounds.y + styling.shadow_offset.y,
                ..bounds
            },
            background: Background::Color(
                [0.0, 0.0, 0.0, 0.5].into(),
            ),
            border_radius: styling.border_radius,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };
        (
            Primitive::Group {
                primitives: vec![shadow, background, footer, body, header],
            },
            if is_mouse_over && !is_disabled {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer> From<Card<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + column::Renderer,
    Message: Clone + 'static,
{
    fn from(card: Card<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(card)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}