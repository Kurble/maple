use crate::draw::Primitive;
use crate::event::Event;
use crate::layout::{Rectangle, Size};
use crate::stylesheet::Stylesheet;
use crate::widget::{Context, IntoNode, Node, Widget};

/// The anchor from which to apply the offset of a `Panel`
#[allow(missing_docs)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// A panel with a fixed size and location within it's parent
pub struct Panel<'a, T> {
    offset: (f32, f32),
    anchor: Anchor,
    content: Node<'a, T>,
}

impl<'a, T: 'a> Panel<'a, T> {
    /// Construct a new `Panel`, with an offset from an anchor
    pub fn new(offset: (f32, f32), anchor: Anchor, content: impl IntoNode<'a, T>) -> Self {
        Self {
            offset,
            anchor,
            content: content.into_node(),
        }
    }

    fn layout(&self, layout: Rectangle) -> Option<Rectangle> {
        let (content_width, content_height) = self.content.size();
        let (left, top) = match self.anchor {
            Anchor::TopLeft => (true, true),
            Anchor::TopRight => (false, true),
            Anchor::BottomLeft => (true, false),
            Anchor::BottomRight => (false, false),
        };
        let available = Rectangle {
            left: if left { layout.left + self.offset.0 } else { layout.left },
            right: if left {
                layout.right
            } else {
                layout.right - self.offset.0
            },
            top: if top { layout.top + self.offset.1 } else { layout.top },
            bottom: if top {
                layout.bottom
            } else {
                layout.bottom - self.offset.1
            },
        };
        if available.left < available.right && available.top < available.bottom {
            let width = match content_width {
                Size::Exact(width) => width.min(available.width()),
                Size::Fill(_) => available.width(),
                Size::Shrink => 0.0,
            };
            let height = match content_height {
                Size::Exact(height) => height.min(available.height()),
                Size::Fill(_) => available.height(),
                Size::Shrink => 0.0,
            };
            Some(Rectangle {
                left: if left { available.left } else { available.right - width },
                right: if left { available.left + width } else { available.right },
                top: if top { available.top } else { available.bottom - height },
                bottom: if top { available.top + height } else { available.bottom },
            })
        } else {
            None
        }
    }
}

impl<'a, T: 'a> Widget<'a, T> for Panel<'a, T> {
    fn widget(&self) -> &'static str {
        "panel"
    }

    fn len(&self) -> usize {
        1
    }

    fn visit_children(&mut self, visitor: &mut dyn FnMut(&mut Node<'a, T>)) {
        visitor(&mut self.content);
    }

    fn size(&self, style: &Stylesheet) -> (Size, Size) {
        (style.width, style.height)
    }

    fn hit(&self, layout: Rectangle, clip: Rectangle, _: &Stylesheet, x: f32, y: f32) -> bool {
        if layout.point_inside(x, y) && clip.point_inside(x, y) {
            self.layout(layout)
                .map(|layout| layout.point_inside(x, y))
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn focused(&self) -> bool {
        self.content.focused()
    }

    fn event(&mut self, layout: Rectangle, clip: Rectangle, _: &Stylesheet, event: Event, context: &mut Context<T>) {
        if let Some(layout) = self.layout(layout) {
            self.content.event(layout, clip, event, context)
        }
    }

    fn draw(&mut self, layout: Rectangle, clip: Rectangle, _: &Stylesheet) -> Vec<Primitive<'a>> {
        if let Some(layout) = self.layout(layout) {
            self.content.draw(layout, clip)
        } else {
            Vec::new()
        }
    }
}

impl<'a, T: 'a> IntoNode<'a, T> for Panel<'a, T> {
    fn into_node(self) -> Node<'a, T> {
        Node::new(self)
    }
}
