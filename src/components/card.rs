use crate::styles::card::StyleSheet;
use iced_graphics::{backend, Backend, Defaults, Primitive};
use iced_native::{
    event,
    layout::{
        flex::{self, Axis},
        Limits, Node,
    },
    mouse, overlay, Align, Background, Clipboard, Color, Element, Event, Hasher, Layout, Length,
    Point, Rectangle, Widget,
};

pub struct Card<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    padding: u16,
    spacing: u16,
    on_pressed: Option<Message>,
    style: Renderer::Style,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Card<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer,
{
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding: Renderer::DEFAULT_PADDING,
            spacing: Renderer::DEFAULT_SPACING,
            on_pressed: None,
            style: Renderer::Style::default(),
            children: Vec::new(),
        }
    }

    pub fn header<E>(mut self, header: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.insert(0, header.into());
        self
    }

    pub fn body<E>(mut self, body: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.insert(1, body.into());
        self
    }

    pub fn footer<E>(mut self, footer: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.insert(2, footer.into());
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

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
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
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = f32::from(self.padding);
        let spacing = f32::from(self.spacing);
        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .max_width(self.max_width)
            .max_height(self.max_height)
            .pad(padding);

        // let header = self.header.layout(&renderer, &limits);
        // let mut body = self.body.layout(&renderer, &limits);
        // body.move_to(Point::new(padding, padding));
        // let footer = self.footer.layout(&renderer, &limits);
        // let size = limits.resolve(body.size()).pad(self.padding as f32);
        // Node::with_children(size, vec![header, body, footer])
        flex::resolve(
            Axis::Vertical,
            renderer,
            &limits.loose(),
            padding,
            spacing,
            Align::Center,
            &self.children,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        self.max_width.hash(state);
        self.max_height.hash(state);
        self.padding.hash(state);
        self.spacing.hash(state);
        self.children.iter().for_each(|e| {
            e.hash_layout(state);
        })
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
            self.on_pressed.is_none(),
            self.state.is_pressed,
            &self.children,
            &self.style,
        )
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.children
            .iter_mut()
            .zip(layout.children())
            .filter_map(|(child, layout)| child.overlay(layout))
            .next()
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
    type Style: Default;

    const DEFAULT_PADDING: u16;
    const DEFAULT_SPACING: u16;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        is_disabled: bool,
        is_pressed: bool,
        children: &[Element<'_, Message, Self>],
        style: &Self::Style,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;

    const DEFAULT_PADDING: u16 = 8;
    const DEFAULT_SPACING: u16 = 8;

    fn draw<Message>(
        &mut self,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        is_disabled: bool,
        is_pressed: bool,
        children: &[Element<'_, Message, Self>],
        style: &Self::Style,
    ) -> Self::Output {
        let layout_bound = layout.bounds();
        let is_mouse_over = layout_bound.contains(cursor_position);

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

        let background = Primitive::Quad {
            bounds: layout_bound,
            background: styling
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: styling.border_radius,
            border_width: styling.border_width,
            border_color: styling.border_color,
        };

        let shadow = Primitive::Quad {
            bounds: Rectangle {
                x: layout_bound.x - styling.shadow_offset.x * 2.,
                y: layout_bound.y - styling.shadow_offset.y,
                width: layout_bound.width + styling.shadow_offset.x * 4.,
                height: layout_bound.height + styling.shadow_offset.y * 2.,
            },
            background: Background::Color([0.0, 0.0, 0.0, 0.2].into()),
            border_radius: styling.border_radius,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };
        (
            Primitive::Group {
                primitives: vec![shadow, background, content],
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
    Renderer: 'a + self::Renderer,
    Message: Clone + 'static,
{
    fn from(card: Card<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(card)
    }
}
