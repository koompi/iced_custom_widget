use iced_graphics::{backend::{self, Backend}, Primitive};
use iced_native::{
    layout::{Limits, Node},
    mouse, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};

pub enum Overflow {
    Visible,
    Clip,
}

impl Overflow {
    pub const ALL: [Overflow; 2] = [
        Overflow::Visible,
        Overflow::Clip
    ];
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}

pub struct Stack<'a, Message, Renderer> {
    overflow: Overflow,
    children: Vec<(Element<'a, Message, Renderer>, Option<Point>)>,
}

impl<'a, Message, Renderer> Stack<'a, Message, Renderer> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<(Element<'a, Message, Renderer>, Option<Point>)>) -> Self {
        Self {
            overflow: Overflow::default(),
            children,
        }
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn push<E>(mut self, element: E, point: Option<Point>) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.push((element.into(), point));
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Stack<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        if self.children.is_empty() {
            Node::new(Size::ZERO)
        } else {
            let mut nodes = Vec::with_capacity(self.children.len());
            let mut height: f32 = 0.;
            let mut width: f32 = 0.;

            for (element, point) in self.children.iter() {
                let size = element.layout(renderer, &limits).size();
                let mut node = Node::new(size);
                if let Some(point) = point {
                    node.move_to(*point);
                    match self.overflow {
                        Overflow::Visible => {
                            width = width.max(size.width + point.x);
                            height = height.max(size.height + point.y);
                        },
                        Overflow::Clip => {
                            height = height.max(size.height);
                            width = width.max(size.width);
                        }
                    }
                    
                } else {
                    height = height.max(size.height);
                    width = width.max(size.width);
                }
                nodes.push(node);
            }

            Node::with_children(Size::new(width, height), nodes)
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
            &self.overflow,
            &self.children,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.children.iter().for_each(|(element, _)| {
            element.hash_layout(state);
        })
    }
}

pub trait Renderer: iced_native::Renderer {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        overflow: &Overflow,
        children: &[(Element<'_, Message, Self>, Option<Point>)],
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend + backend::Text,
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        overflow: &Overflow,
        children: &[(Element<'_, Message, Self>, Option<Point>)],
    ) -> Self::Output {
        let layout_bound = layout.bounds();
        let is_mouse_over = layout_bound.contains(cursor_position);
        let mut mouse_interaction = mouse::Interaction::default();
        (
            Primitive::Group {
                primitives: children
                    .iter()
                    .zip(layout.children())
                    .map(|((element, _), layout)| {
                        let (primitive, new_mouse_interaction) =
                            element.draw(self, defaults, layout, cursor_position, viewport);

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            },
        )
    }
}

impl<'a, Message, Renderer> From<Stack<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: Clone + 'a,
{
    fn from(stack: Stack<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(stack)
    }
}
