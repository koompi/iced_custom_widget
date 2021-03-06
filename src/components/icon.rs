use iced_graphics::{backend, Backend, Primitive};
use iced_native::{
    layout::{Limits, Node},
    mouse, Color, Element, Font, Hasher, HorizontalAlignment, Layout, Length, Point, Rectangle,
    Size, VerticalAlignment, Widget,
};
pub use crate::utils::Icons;

const ICON_FONT: Font = Font::External {
    name: "Line Awesome",
    bytes: include_bytes!("../../assets/fonts/la-solid-900.ttf"),
};

pub struct Icon {
    icon: Icons,
    width: Length,
    height: Length,
    color: Option<Color>,
    font: Font,
    size: Option<u16>,
    label: String,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl Icon {
    pub fn new(icon: Icons) -> Self {
        Self {
            icon,
            width: Length::Shrink,
            height: Length::Shrink,
            color: None,
            font: ICON_FONT,
            size: None,
            label: String::new(),
            horizontal_alignment: HorizontalAlignment::Center,
            vertical_alignment: VerticalAlignment::Center,
        }
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn font<F: Into<Font>>(mut self, font: F) -> Self {
        self.font = font.into();
        self
    }

    pub fn size(mut self, size: u16) -> Self {
        self.size = Some(size);
        self
    }

    pub fn label<L: Into<String>>(mut self, label: L) -> Self {
        self.label = label.into();
        self
    }

    pub fn horizontal_alignment(mut self, alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn vertical_alignment(mut self, alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Icon
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
        let size = self.size.unwrap_or(renderer.default_size());
        let bounds = limits.max();
        let (width, height) = renderer.measure(self.icon, size, self.font, bounds);
        let size = limits.resolve(Size::new(width, height));
        Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            layout.bounds(),
            viewport,
            self.icon,
            self.size.unwrap_or(renderer.default_size()),
            self.font,
            self.color,
            &self.label,
            self.horizontal_alignment,
            self.vertical_alignment,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.icon.hash(state);
        self.size.hash(state);
    }
}

impl<'a, Message, Renderer> From<Icon> for Element<'a, Message, Renderer>
where
    Renderer: self::Renderer + 'a,
{
    fn from(icon: Icon) -> Element<'a, Message, Renderer> {
        Element::new(icon)
    }
}

pub trait Renderer: iced_native::Renderer {
    type Font: Default + Copy;

    fn default_size(&self) -> u16;

    fn measure(&self, icon: Icons, size: u16, font: Font, bounds: Size) -> (f32, f32);

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        viewport: &Rectangle,
        icon: Icons,
        size: u16,
        font: Font,
        color: Option<Color>,
        label: &str,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Text,
{
    type Font = Font;
    fn default_size(&self) -> u16 {
        self.backend().default_size()
    }

    fn measure(&self, content: Icons, size: u16, font: Font, bounds: Size) -> (f32, f32) {
        self.backend()
            .measure(&content.to_string(), f32::from(size), font, bounds)
    }

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        _viewport: &Rectangle,
        content: Icons,
        size: u16,
        font: Font,
        color: Option<Color>,
        _label: &str,
        horizontal_alignment: HorizontalAlignment,
        vertical_alignment: VerticalAlignment,
    ) -> Self::Output {
        let x = match horizontal_alignment {
            iced_native::HorizontalAlignment::Left => bounds.x,
            iced_native::HorizontalAlignment::Center => bounds.center_x(),
            iced_native::HorizontalAlignment::Right => bounds.x + bounds.width,
        };

        let y = match vertical_alignment {
            iced_native::VerticalAlignment::Top => bounds.y,
            iced_native::VerticalAlignment::Center => bounds.center_y(),
            iced_native::VerticalAlignment::Bottom => bounds.y + bounds.height,
        };

        (
            Primitive::Text {
                content: content.to_string(),
                size: f32::from(size),
                bounds: Rectangle { x, y, ..bounds },
                color: color.unwrap_or(defaults.text.color),
                font,
                horizontal_alignment,
                vertical_alignment,
            },
            mouse::Interaction::default(),
        )
    }
}

impl Clone for Icon {
    fn clone(&self) -> Self {
        Self {
            icon: self.icon,
            width: self.width,
            height: self.height,
            size: self.size,
            color: self.color,
            font: self.font,
            label: self.label.clone(),
            horizontal_alignment: self.horizontal_alignment,
            vertical_alignment: self.vertical_alignment,
        }
    }
}
