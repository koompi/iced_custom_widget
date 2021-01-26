//! Create choices using tab buttons.
use crate::styles::tab::{Position, StyleSheet};
use iced_graphics::defaults::{self, Defaults};
use iced_graphics::{Backend, Primitive};
use iced_native::event::{self, Event};
use iced_native::layout;
use iced_native::mouse;
use iced_native::{
    text, Background, Clipboard, Color, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};
use std::hash::Hash;

#[allow(missing_debug_implementations)]
pub struct Tab<'a, Message, Renderer: self::Renderer> {
    content: Element<'a, Message, Renderer>,
    is_selected: bool,
    on_click: Message,
    width: Length,
    height: Length,
    min_width: u32,
    min_height: u32,
    padding: u16,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer> Tab<'a, Message, Renderer>
where
    Message: Clone,
{
    /// Creates a new [`Tab`] button.
    ///
    /// It expects:
    ///   * the value related to the [`Tab`] button
    ///   * the current selected value
    ///   * a function that will be called when the [`Tab`] is selected. It
    ///   receives the value of the tab and must produce a `Message`.
    ///   * the content of to display in the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn new<F, V, E>(value: V, selected: Option<V>, f: F, content: E) -> Self
    where
        V: Eq + Copy,
        F: 'static + Fn(V) -> Message,
        E: Into<Element<'a, Message, Renderer>>,
    {
        Tab {
            content: content.into(),
            is_selected: Some(value) == selected,
            on_click: f(value),
            width: Length::Shrink,
            height: Length::Shrink,
            min_width: 0,
            min_height: 0,
            padding: Renderer::DEFAULT_PADDING,
            style: Renderer::Style::default(),
        }
    }
    /// Sets the width of the [`Tab`] button.
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
    /// Sets the minimum width of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Sets the minimum height of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Sets the padding of the [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the style of the [`Tab`] button.
    ///
    /// [`Tab`]: struct.Tab.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Tab<'a, Message, Renderer>
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

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let padding = f32::from(self.padding);
        let limits = limits
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits);
        content.move_to(Point::new(padding, padding));

        let size = limits.resolve(content.size()).pad(padding);

        layout::Node::with_children(size, vec![content])
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
                if layout.bounds().contains(cursor_position) {
                    messages.push(self.on_click.clone());

                    return event::Status::Captured;
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
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            layout.bounds(),
            cursor_position,
            self.is_selected,
            &self.style,
            &self.content,
            layout.children().next().unwrap(),
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.content.hash_layout(state);
    }
}

/// The renderer of a [`Tab`] button.
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Tab`] button in your user interface.
///
/// [`Tab`]: struct.Tab.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: iced_native::Renderer {
    /// The default padding of a [`Tab`].
    ///
    /// [`Tab`]: struct.Tab.html
    const DEFAULT_PADDING: u16;

    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Button`].
    ///
    /// [`Button`]: struct.Button.html
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_selected: bool,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output;
}

// impl<'a, Message, Renderer> From<Tab<Message, Renderer>> for Tab<'a, Message, Renderer>
// where
//     Renderer: 'a + self::Renderer + text::Renderer,
//     Message: 'a,
// {
//     fn from(tab: Tab<Message, Renderer>) -> Self {

//     }
// }

impl<'a, Message, Renderer> From<Tab<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer,
    Message: 'a + Clone,
{
    fn from(tab: Tab<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(tab)
    }
}
impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 5;

    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        _defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_selected: bool,
        style_sheet: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_mouse_over {
            if is_selected {
                style_sheet.selected_hovered()
            } else {
                style_sheet.unselected_hovered()
            }
        } else {
            if is_selected {
                style_sheet.selected()
            } else {
                style_sheet.unselected()
            }
        };

        let (content, _) = content.draw(
            self,
            &Defaults {
                text: defaults::Text {
                    color: style.text_color,
                },
            },
            content_layout,
            cursor_position,
            &bounds,
        );

        let indicator = if let Some(indicator_style) = style.indicator {
            if indicator_style.position == Position::Bottom
                || indicator_style.position == Position::Top
            {
                let (x, width) = if let Some(length) = indicator_style.length {
                    (
                        bounds.x + ((bounds.width - f32::from(length)) / 2.0),
                        f32::from(length),
                    )
                } else {
                    (bounds.x, bounds.width)
                };

                let y = if indicator_style.position == Position::Bottom {
                    bounds.y + bounds.height
                        - indicator_style.thickness
                        - f32::from(indicator_style.offset)
                } else {
                    bounds.y + f32::from(indicator_style.offset)
                };

                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y,
                        width,
                        height: indicator_style.thickness,
                    },
                    background: Background::Color(indicator_style.color),
                    border_radius: indicator_style.border_radius,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            } else {
                let (y, height) = if let Some(length) = indicator_style.length {
                    (
                        bounds.y + ((bounds.height - f32::from(length)) / 2.0),
                        f32::from(length),
                    )
                } else {
                    (bounds.y, bounds.height)
                };

                let x = if indicator_style.position == Position::Right {
                    bounds.x + bounds.width
                        - indicator_style.thickness
                        - f32::from(indicator_style.offset)
                } else {
                    bounds.x + f32::from(indicator_style.offset)
                };

                Primitive::Quad {
                    bounds: Rectangle {
                        x,
                        y,
                        width: indicator_style.thickness,
                        height,
                    },
                    background: Background::Color(indicator_style.color),
                    border_radius: indicator_style.border_radius,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                }
            }
        } else {
            Primitive::None
        };

        (
            if style.background.is_some() || style.border_width > 0.0 {
                let background = Primitive::Quad {
                    bounds,
                    background: style
                        .background
                        .unwrap_or(Background::Color(Color::TRANSPARENT)),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                };

                Primitive::Group {
                    primitives: vec![background, indicator, content],
                }
            } else {
                Primitive::Group {
                    primitives: vec![indicator, content],
                }
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}
