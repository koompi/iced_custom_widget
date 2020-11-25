use iced_graphics::Backend;
use iced_native::{
    layout::{Limits, Node},
    mouse, text, Element, Hasher, Layout, Length, Point, Rectangle, Text, Widget,
};
pub struct TableCell<'a, Message, Renderer> {
    state: &'a mut State,
    content: Element<'a, Message, Renderer>,
    placeholder: Option<&'a str>,
    padding: u16,
}

pub struct State {
    width: Length,
}

impl<'a, Message, Renderer> TableCell<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + text::Renderer,
{
    pub fn new(state: &'a mut State, content: Text<Renderer>) -> Self {
        Self {
            state,
            content: content.into(),
            placeholder: None,
            padding: Renderer::DEFAULT_PADDING,
        }
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TableCell<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        self.state.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        let padding = f32::from(self.padding);
        let limits = limits
            .width(self.state.width)
            .height(self.height())
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits);
        content.move_to(Point::new(padding, padding));

        let size = limits.resolve(content.size()).pad(padding);

        Node::with_children(size, vec![content])
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            defaults,
            layout.bounds(),
            cursor_position,
            &self.content,
            layout.children().next().unwrap(),
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.content.hash_layout(state);
    }
}

pub trait Renderer: iced_native::Renderer {
    const DEFAULT_PADDING: u16;

    fn draw<Message>(
        &mut self,
        default: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 5;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let (content, _) = content.draw(self, defaults, content_layout, cursor_position, &bounds);

        (content, mouse::Interaction::default())
    }
}

impl<'a, Message, Renderer> From<TableCell<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + self::Renderer,
{
    fn from(table_cell: TableCell<'a, Message, Renderer>) -> Self {
        Element::new(table_cell)
    }
}
