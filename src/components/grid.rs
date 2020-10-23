use iced_native::{
    layout::{
        flex::{self, Axis},
        Limits, Node,
    },
    mouse, Align, Element, Hasher, Layout, Length, Point, Size, Widget,
};
// use iced_wgpu::{Primitive, Defaults, Renderer};
use std::{any::TypeId, hash::Hash, iter};

#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Renderer> {
    columns: Option<usize>,
    column_width: Option<u16>,
    elements: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer> {
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(elements: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            columns: None,
            column_width: None,
            elements,
        }
    }

    pub fn columns(mut self, columns: usize) -> Self {
        if columns == 0 {
            self.columns = None;
        } else {
            self.columns = Some(columns);
        }

        self
    }

    pub fn column_width(mut self, column_width: u16) -> Self {
        self.column_width = Some(column_width);
        self
    }

    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Grid<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn width(&self) -> Length {
        if self.columns.is_some() {
            Length::Shrink
        } else {
            Length::Fill
        }
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        if self.elements.is_empty() {
            Node::new(Size::ZERO)
        } else {
            let column_limits = if let Some(column_width) = self.column_width {
                limits.width(Length::Units(column_width))
            } else {
                *limits
            };

            // if we have a given number of columns, we can find out how
            // wide a column is by finding the widest cell in it
            if let Some(columns) = self.columns {
                // store calculated layout sizes
                let mut layouts = Vec::with_capacity(self.elements.len());
                // store width of each column
                let mut column_widths = Vec::<f32>::with_capacity(columns);

                for (column, element) in (0..columns).cycle().zip(&self.elements) {
                    let layout = element.layout(renderer, &column_limits).size();
                    layouts.push(layout);

                    if let Some(column_width) = column_widths.get_mut(column) {
                        *column_width = column_width.max(layout.width);
                    } else {
                        column_widths.insert(column, layout.width);
                    }
                }

                // list of x alignments for every column
                let column_aligns = iter::once(&0.)
                    .chain(column_widths.iter().take(column_widths.len() - 1))
                    .scan(0., |state, width| {
                        *state += width;
                        Some(*state)
                    });

                let mut nodes = Vec::with_capacity(self.elements.len());
                let mut grid_height = 0.;
                let mut row_height = 0.;

                for ((column, column_align), size) in column_aligns.enumerate().cycle().zip(layouts)
                {
                    if column == 0 {
                        grid_height += row_height;
                        row_height = 0.;
                    }

                    let mut node = Node::new(size);
                    node.move_to(Point::new(column_align, grid_height));
                    nodes.push(node);
                    row_height = row_height.max(size.height);
                }

                grid_height += row_height;
                let grid_width = column_widths.into_iter().sum();

                Node::with_children(Size::new(grid_width, grid_height), nodes)
            // if we have `column_width` but no `columns`, calculate number of
            // columns by checking how many can fit
            } else if let Some(column_width) = self.column_width {
                let column_width: f32 = column_width.into();
                let max_width = limits.max().width;
                let columns = (max_width / column_width).floor() as usize;
                let mut nodes = Vec::with_capacity(self.elements.len());
                let mut grid_height = 0.;
                let mut row_height = 0.;

                for (column, element) in (0..columns).cycle().zip(&self.elements) {
                    if column == 0 {
                        grid_height += row_height;
                        row_height = 0.;
                    }

                    let size = element.layout(renderer, &column_limits).size();
                    let mut node = Node::new(size);
                    node.move_to(Point::new(column as f32 * column_width, grid_height));
                    nodes.push(node);
                    row_height = row_height.max(size.height);
                }

                grid_height += row_height;
                let grid_width = (columns as f32) * column_width;

                Node::with_children(Size::new(grid_width, grid_height), nodes)
            // if we didn't define `columns` and `column_width` just put them
            // horizontally next to each other
            } else {
                flex::resolve(
                    Axis::Horizontal,
                    renderer,
                    &limits,
                    0.,
                    0.,
                    Align::Start,
                    &self.elements,
                )
            }
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(defaults, layout, cursor_position, &self.elements)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        TypeId::of::<Grid<'_, (), ()>>().hash(state);

        for element in &self.elements {
            element.hash_layout(state);
        }
    }
}

pub trait Renderer: iced_native::Renderer + Sized {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        elements: &[Element<'_, Message, Self>],
    ) -> Self::Output;
}

impl Renderer for iced_wgpu::Renderer {
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        content: &[Element<'_, Message, Self>],
    ) -> Self::Output {
        let mut mouse_interaction = mouse::Interaction::default();

        (
            iced_wgpu::Primitive::Group {
                primitives: content
                    .iter()
                    .zip(layout.children())
                    .map(|(child, layout)| {
                        let (primitive, new_mouse_interaction) =
                            child.draw(self, defaults, layout, cursor_position);

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse_interaction,
        )
    }
}

impl<'a, Message, Renderer> From<Grid<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'static,
{
    fn from(grid: Grid<'a, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(grid)
    }
}
