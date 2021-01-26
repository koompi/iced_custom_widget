//! Show toggle controls using togglers.
use std::hash::Hash;

use iced_native::event;
use iced_native::{
    layout, mouse, row, text, Align, Clipboard, Element, Event, Hasher, HorizontalAlignment,
    Layout, Length, Point, Rectangle, Row, Text, VerticalAlignment, Widget,
};

/// A toggler widget
#[allow(missing_debug_implementations)]
pub struct Toggler<Message, Renderer: self::Renderer + text::Renderer> {
    is_active: bool,
    on_toggle: Box<dyn Fn(bool) -> Message>,
    label: Option<String>,
    width: Length,
    size: u16,
    text_size: Option<u16>,
    text_align: Option<HorizontalAlignment>,
    spacing: u16,
    font: Renderer::Font,
    style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer + text::Renderer> Toggler<Message, Renderer> {
    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn new<F>(is_active: bool, label: impl Into<Option<String>>, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Toggler {
            is_active,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Fill,
            size: <Renderer as self::Renderer>::DEFAULT_SIZE,
            text_size: None,
            text_align: None,
            spacing: 0,
            font: Renderer::Font::default(),
            style: Renderer::Style::default(),
        }
    }

    /// Sets the size of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the width of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the text size o the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the horizontal alignment of the text of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn text_align(mut self, align: HorizontalAlignment) -> Self {
        self.text_align = Some(align);
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the [`Font`] of the text of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    /// [`Font`]: ../../struct.Font.html
    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Toggler<Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }
    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let mut row = Row::<(), Renderer>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Align::Center);

        if let Some(label) = &self.label {
            row = row.push(
                Text::new(label)
                    .horizontal_alignment(self.text_align.unwrap_or(HorizontalAlignment::Left))
                    .font(self.font)
                    .width(self.width)
                    .size(self.text_size.unwrap_or(renderer.default_size())),
            );
        }

        row = row.push(
            Row::new()
                .width(Length::Units(2 * self.size))
                .height(Length::Units(self.size)),
        );

        row.layout(renderer, limits)
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
        let status = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = layout.bounds().contains(cursor_position);
                if mouse_over {
                    messages.push((self.on_toggle)(!self.is_active));
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        };

        status
    }
    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let label = match &self.label {
            Some(label) => {
                let label_layout = children.next().unwrap();

                Some(text::Renderer::draw(
                    renderer,
                    defaults,
                    label_layout.bounds(),
                    &label,
                    self.text_size.unwrap_or(renderer.default_size()),
                    self.font,
                    None,
                    self.text_align.unwrap_or(HorizontalAlignment::Left),
                    VerticalAlignment::Center,
                ))
            }

            None => None,
        };

        let toggler_layout = children.next().unwrap();
        let toggler_bounds = toggler_layout.bounds();

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            toggler_bounds,
            self.is_active,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.label.hash(state)
    }
}

/// The renderer of a [`Toggler`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Toggler`] in your user interface.
///
/// [`Toggler`]: struct.Toggler.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: iced_native::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// The default size of a [`Toggler`].
    ///
    /// [`Toggler`]: struct.Toggler.html
    const DEFAULT_SIZE: u16;

    /// Draws a [`Toggler`].
    ///
    /// It receives:
    ///   * the bounds of the [`Toggler`]
    ///   * whether the [`Toggler`] is activated or not
    ///   * whether the mouse is over the [`Toggler`] or not
    ///   * the drawn label of the [`Toggler`]
    ///   * the style of the [`Toggler`]
    ///
    /// [`Toggler`]: struct.Toggler.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        is_active: bool,
        is_mouse_over: bool,
        label: Option<Self::Output>,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Toggler<Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer + row::Renderer,
    Message: 'a,
{
    fn from(toggler: Toggler<Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(toggler)
    }
}

mod graphics {
    /// Makes sure that the border radius of the toggler looks good at every size.
    const BORDER_RADIUS_RATIO: f32 = 32.0 / 13.0;

    /// The space ratio between the background Quad and the Toggler bounds, and
    /// between the background Quad and foreground Quad.
    const SPACE_RATIO: f32 = 0.05;
    use crate::styles::toggler::StyleSheet;
    use crate::toggler;
    use iced_graphics::backend::{self, Backend};
    use iced_graphics::{Primitive, Renderer};
    use iced_native::mouse;
    use iced_native::Rectangle;
    impl<B> toggler::Renderer for Renderer<B>
    where
        B: Backend + backend::Text,
    {
        type Style = Box<dyn StyleSheet>;
        const DEFAULT_SIZE: u16 = 20;
        fn draw(
            &mut self,
            bounds: Rectangle,
            is_active: bool,
            is_mouse_over: bool,
            label: Option<Self::Output>,
            style_sheet: &Self::Style,
        ) -> Self::Output {
            let style = if is_mouse_over {
                style_sheet.hovered(is_active)
            } else {
                style_sheet.active(is_active)
            };
            let border_radius = (bounds.height as f32 / BORDER_RADIUS_RATIO) as u16;
            let space = SPACE_RATIO * bounds.height as f32;
            let toggler_background_bounds = Rectangle {
                x: bounds.x + space,
                y: bounds.y + space,
                width: bounds.width - (2.0 * space),
                height: bounds.height - (2.0 * space),
            };
            let toggler_background = Primitive::Quad {
                bounds: toggler_background_bounds,
                background: style.background.into(),
                border_radius: border_radius as f32,
                border_width: 1.,
                border_color: style.background_border.unwrap_or(style.background),
            };
            let toggler_foreground_bounds = Rectangle {
                x: bounds.x
                    + if is_active {
                        bounds.width - 2.0 * space - (bounds.height - (4.0 * space))
                    } else {
                        2.0 * space
                    },
                y: bounds.y + (2.0 * space),
                width: bounds.height - (4.0 * space),
                height: bounds.height - (4.0 * space),
            };
            let toggler_foreground = Primitive::Quad {
                bounds: toggler_foreground_bounds,
                background: style.foreground.into(),
                border_radius: border_radius as f32,
                border_width: 1.,
                border_color: style.foreground_border.unwrap_or(style.foreground),
            };
            (
                Primitive::Group {
                    primitives: match label {
                        Some((l, _)) => {
                            vec![l, toggler_background, toggler_foreground]
                        }
                        None => vec![toggler_background, toggler_foreground],
                    },
                },
                if is_mouse_over {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                },
            )
        }
    }
}
