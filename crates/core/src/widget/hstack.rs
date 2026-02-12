use crate::{Margin, Widget, id::next_id, style::Style};

#[derive(Debug)]
pub struct HStack<M: Clone> {
    pub children: Vec<Widget<M>>,
    pub spacing: i32,
    pub margin: Margin,
}

impl<M: Clone> HStack<M> {
    pub fn new(children: Vec<Widget<M>>) -> Self {
        Self {
            children,
            spacing: 0,
            margin: Margin::new(),
        }
    }

    pub fn extend(&mut self, children: Vec<Widget<M>>) {
        self.children = children;
    }

    pub fn push(&mut self, child: Widget<M>) {
        self.children.push(child);
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn build(self) -> Widget<M> {
        // HStack style
        let hstack_style = Style {
            spacing: self.spacing,
            margin: self.margin,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::HStack {
                children: self.children,
            },
            on_press: None,
            style: hstack_style,
        }
    }
}
