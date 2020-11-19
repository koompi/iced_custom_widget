use crate::styles::stepper::StyleSheet;
use iced_graphics::{backend, Backend, Primitive};
use iced_native::{
    button, container,
    event::{self, Event},
    layout::{Limits, Node},
    mouse, row, text, Align, Background, Clipboard, Color, Container, Element, Hasher,
    HorizontalAlignment, Layout, Length, Point, Rectangle, Row, Text, VerticalAlignment, Widget,
};

pub struct Stepper<'a, Message, Renderer: self::Renderer + text::Renderer> {
    value: f32,
    step: f32,
    min: f32,
    max: f32,
    spacing: u16,
    padding: u16,
    value_width: Option<u16>,
    text_size: Option<u16>,
    decrease_btn_state: &'a mut State,
    increase_btn_state: &'a mut State,
    on_changed: Box<dyn Fn(f32) -> Message + 'a>,
    font: Renderer::Font,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> Stepper<'a, Message, Renderer>
where
    Renderer: text::Renderer + self::Renderer,
{
    pub fn new<F>(
        value: f32,
        decrease_btn_state: &'a mut State,
        increase_btn_state: &'a mut State,
        on_changed: F,
    ) -> Self
    where
        F: 'static + Fn(f32) -> Message,
    {
        Self {
            value,
            step: 1.,
            min: 0.,
            max: 100.,
            spacing: 0,
            padding: Renderer::DEFAULT_PADDING,
            value_width: None,
            text_size: None,
            decrease_btn_state,
            increase_btn_state,
            on_changed: Box::new(on_changed),
            font: Renderer::Font::default(),
            style: Renderer::Style::default(),
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    pub fn value_width(mut self, value_width: u16) -> Self {
        self.value_width = Some(value_width);
        self
    }

    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = font;
        self
    }

    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Stepper<'a, Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer + container::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .width(Length::Shrink)
            .height(Length::Shrink)
            .pad(f32::from(self.padding));
        let mut value = Container::<(), Renderer>::new(
            Text::new(format!("{:.2}", self.value)).size(renderer.default_size()),
        )
        .center_y()
        .center_x()
        .padding(self.padding);

        if let Some(width) = self.value_width {
            value = value.width(Length::Units(width));
        }

        // size is width & height of button (text_size + padding * 2)
        let size = self
            .text_size
            .unwrap_or(renderer.default_size() + (self.padding * 2));
        Row::<(), Renderer>::new()
            .width(Length::Shrink)
            .spacing(self.spacing)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .padding(self.padding)
                    .width(Length::Units(size))
                    .height(Length::Units(size)),
            )
            .push(value)
            .push(
                Row::new()
                    .padding(self.padding)
                    .width(Length::Units(size))
                    .height(Length::Units(size)),
            )
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
        let mut children = layout.children();
        let decrease_btn_bounds = children.next().unwrap().bounds();
        let value_bounds = children.next().unwrap().bounds();
        let increase_btn_bounds = children.next().unwrap().bounds();

        let value = text::Renderer::draw(
            renderer,
            defaults,
            value_bounds,
            format!("{:.2}", self.value).as_str(),
            self.text_size.unwrap_or(renderer.default_size()),
            self.font,
            None,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
        );
        let is_decrease_disabled = self.value == self.min;
        let is_increase_disabled = self.value == self.max;

        self::Renderer::draw(
            renderer,
            cursor_position,
            self.decrease_btn_state.is_pressed,
            is_decrease_disabled,
            self.increase_btn_state.is_pressed,
            is_increase_disabled,
            decrease_btn_bounds,
            value_bounds,
            increase_btn_bounds,
            value,
            &self.font,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.padding.hash(state);
        self.spacing.hash(state);
        self.text_size.hash(state);
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
        let mut event_status = event::Status::Ignored;
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = layout.bounds().contains(cursor_position);

                if mouse_over {
                    event_status = event::Status::Captured;
                    let mut children = layout.children();
                    let decrease_btn_layout = children.next().unwrap();
                    let _value_layout = children.next().unwrap();
                    let increase_btn_layout = children.next().unwrap();

                    if decrease_btn_layout.bounds().contains(cursor_position) {
                        self.decrease_btn_state.is_pressed = true;
                        self.value -= self.step;
                        if self.value < self.min {
                            self.value = self.min
                        }
                        messages.push((self.on_changed)(self.value));
                    } else if increase_btn_layout.bounds().contains(cursor_position) {
                        self.increase_btn_state.is_pressed = true;
                        self.value += self.step;
                        if self.value > self.max {
                            self.value = self.max
                        }
                        messages.push((self.on_changed)(self.value));
                    } else {
                        event_status = event::Status::Ignored;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let bounds = layout.bounds();
                if bounds.contains(cursor_position) {
                    event_status = event::Status::Captured;
                    let mut children = layout.children();
                    let decrease_btn_layout = children.next().unwrap();
                    let _value_layout = children.next().unwrap();
                    let increase_btn_layout = children.next().unwrap();
                    if decrease_btn_layout.bounds().contains(cursor_position) {
                        self.decrease_btn_state.is_pressed = false;
                    } else if increase_btn_layout.bounds().contains(cursor_position) {
                        self.increase_btn_state.is_pressed = false;
                    } else {
                        event_status = event::Status::Ignored;
                    }
                }
            }
            _ => {}
        }
        event_status
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct State {
    is_pressed: bool,
}

pub trait Renderer: iced_native::Renderer + text::Renderer {
    type Style: Default;
    const DEFAULT_PADDING: u16;
    fn draw(
        &mut self,
        cursor_position: Point,
        is_decrease_pressed: bool,
        is_decrease_disabled: bool,
        is_increase_pressed: bool,
        is_increase_disabled: bool,
        decrease_btn_bounds: Rectangle,
        value_bounds: Rectangle,
        increase_btn_bounds: Rectangle,
        value: Self::Output,
        font: &Self::Font,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Text,
{
    type Style = Box<dyn StyleSheet>;
    const DEFAULT_PADDING: u16 = 8;
    fn draw(
        &mut self,
        cursor_position: Point,
        is_decrease_pressed: bool,
        is_decrease_disabled: bool,
        is_increase_pressed: bool,
        is_increase_disabled: bool,
        decrease_btn_bounds: Rectangle,
        value_bounds: Rectangle,
        increase_btn_bounds: Rectangle,
        (value, _): Self::Output,
        font: &Self::Font,
        style: &Self::Style,
    ) -> Self::Output {
        let mouse_over_decrease = decrease_btn_bounds.contains(cursor_position);
        let mouse_over_increase = increase_btn_bounds.contains(cursor_position);

        let decrease_btn_style = if is_decrease_disabled {
            style.disabled()
        } else if is_decrease_pressed {
            style.pressed()
        } else if mouse_over_decrease {
            style.hovered()
        } else {
            style.active()
        };

        let increase_btn_style = if is_increase_disabled {
            style.disabled()
        } else if is_increase_pressed {
            style.pressed()
        } else if mouse_over_increase {
            style.hovered()
        } else {
            style.active()
        };

        // decrease button section
        let decrease_button_rect = Primitive::Quad {
            bounds: decrease_btn_bounds,
            background: decrease_btn_style
                .button_background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: decrease_btn_style.border_radius,
            border_width: decrease_btn_style.border_width,
            border_color: decrease_btn_style.border_color,
        };
        let decrease_text = Primitive::Text {
            content: String::from("-"),
            bounds: Rectangle {
                x: decrease_btn_bounds.center_x(),
                y: decrease_btn_bounds.center_y(),
                ..decrease_btn_bounds
            },
            font: *font,
            size: decrease_btn_bounds.height * 0.7,
            color: decrease_btn_style.text_color,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        };
        let decrease_btn = Primitive::Group {
            primitives: vec![decrease_button_rect, decrease_text],
        };

        // current value container section
        let value_rect = Primitive::Quad {
            bounds: value_bounds,
            background: decrease_btn_style
                .text_background
                .unwrap_or(Background::Color([0.7, 0.7, 0.7].into())),
            border_radius: 0,
            border_width: 0,
            border_color: Color::TRANSPARENT,
        };
        let value_container = Primitive::Group {
            primitives: vec![value_rect, value],
        };

        // increase button section
        let increase_button_rect = Primitive::Quad {
            bounds: increase_btn_bounds,
            background: increase_btn_style
                .button_background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: increase_btn_style.border_radius,
            border_width: increase_btn_style.border_width,
            border_color: increase_btn_style.border_color,
        };
        let increase_text = Primitive::Text {
            content: String::from("+"),
            bounds: Rectangle {
                x: increase_btn_bounds.center_x(),
                y: increase_btn_bounds.center_y(),
                ..increase_btn_bounds
            },
            font: *font,
            size: increase_btn_bounds.height * 0.7,
            color: increase_btn_style.text_color,
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        };
        let increase_btn = Primitive::Group {
            primitives: vec![increase_button_rect, increase_text],
        };

        (
            Primitive::Group {
                primitives: vec![decrease_btn, value_container, increase_btn],
            },
            if mouse_over_decrease || mouse_over_increase {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer> From<Stepper<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a
        + self::Renderer
        + text::Renderer
        + container::Renderer
        + row::Renderer
        + button::Renderer,
{
    fn from(stepper: Stepper<'a, Message, Renderer>) -> Self {
        Element::new(stepper)
    }
}