use iced_graphics::{Backend, Defaults, Primitive};
use iced_native::{
    layout::{
        flex::{self, Axis},
        Limits, Node, 
    }, 
    mouse, Align, Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget, Event, Clipboard, overlay
};
use std::{any::TypeId, hash::Hash, iter};

pub struct Grid<'a, Message, Renderer> {
    padding: u16,
    columns: Option<usize>,
    column_width: Option<u16>,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    pub fn with_children(children: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            padding: Renderer::DEFAULT_PADDING,
            columns: None,
            column_width: None,
            children,
        }
    }

    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
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

    pub fn push<E>(mut self, children: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.push(children.into());
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
        if self.children.is_empty() {
            Node::new(Size::ZERO)
        } else {
            let padding = f32::from(self.padding);
            let column_limits = if let Some(column_width) = self.column_width {
                limits.width(Length::Units(column_width))
            } else {
                *limits
            };
            // if we have a given number of columns, we can find out how
            // wide a column is by finding the widest cell in it
            if let Some(columns) = self.columns {
                // store calculated layout sizes
                let mut layouts = Vec::with_capacity(self.children.len());
                // store width of each column
                let mut column_widths = Vec::<f32>::with_capacity(columns);

                for (column, element) in (0..columns).cycle().zip(&self.children) {
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

                let mut nodes = Vec::with_capacity(self.children.len());
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
                let column_width = f32::from(column_width);
                let max_width = limits.max().width;
                let columns = (max_width / column_width).floor() as u16;
                let mut nodes = Vec::with_capacity(self.children.len());
                let mut grid_height = 0.;
                let mut row_height = 0.;

                for (column, element) in (0..columns).cycle().zip(&self.children) {
                    if column == 0 {
                        grid_height += row_height;
                        row_height = 0.;
                    }

                    let size = element.layout(renderer, &column_limits).size();
                    let mut node = Node::new(size);
                    node.move_to(Point::new((column as f32) * column_width, grid_height));
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
                    padding,
                    8.,
                    Align::Start,
                    &self.children,
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
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(defaults, layout, cursor_position, viewport, &self.children)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        TypeId::of::<Grid<'_, (), ()>>().hash(state);

        self.padding.hash(state);
        for e in &self.children {
            e.hash_layout(state);
        }
    }

    // fn on_event(
    //     &mut self,
    //     event: Event,
    //     layout: Layout<'_>,
    //     cursor_position: Point,
    //     messages: &mut Vec<Message>,
    //     renderer: &Renderer,
    //     clipboard: Option<&dyn Clipboard>,
    // ) {
    //     self.children.iter_mut().zip(layout.children()).for_each(
    //         |(child, layout)| {
    //             child.widget.on_event(
    //                 event.clone(),
    //                 layout,
    //                 cursor_position,
    //                 messages,
    //                 renderer,
    //                 clipboard,
    //             )
    //         },
    //     );
    // }

    // fn overlay(
    //     &mut self,
    //     layout: Layout<'_>,
    // ) -> Option<overlay::Element<'_, Message, Renderer>> {
    //     self.children
    //         .iter_mut()
    //         .zip(layout.children())
    //         .filter_map(|(child, layout)| child.widget.overlay(layout))
    //         .next()
    // }
}

pub trait Renderer: iced_native::Renderer + Sized{
    const DEFAULT_PADDING: u16;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        children: &[Element<'_, Message, Self>],
    ) -> Self::Output;
}

impl<B> Renderer for iced_graphics::Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 8;

    fn draw<Message>(
        &mut self,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        children: &[Element<'_, Message, Self>],
    ) -> Self::Output {
        let mut mouse_interaction = mouse::Interaction::default();

        (
            Primitive::Group {
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
