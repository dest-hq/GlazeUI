use std::marker::PhantomData;

use crate::{Margin, Widget, id::next_id, style::Style};

#[derive(Debug)]
pub struct VStack<M: Clone, App: 'static> {
    pub children: Vec<Widget<M, App>>,
    pub margin: Margin,
    pub spacing: i32,
}

impl<M: Clone, App> VStack<M, App> {
    pub fn new(children: Vec<Widget<M, App>>) -> Self {
        Self {
            children,
            margin: Margin::new(),
            spacing: 0,
        }
    }

    pub fn extend(&mut self, children: Vec<Widget<M, App>>) {
        self.children = children;
    }

    pub fn push(&mut self, child: Widget<M, App>) {
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

    pub fn build(self) -> Widget<M, App> {
        // VStack style
        let vstack_style = Style {
            spacing: self.spacing,
            margin: self.margin,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::VStack {
                children: self.children,
            },
            on_press: None,
            style: vstack_style,
            _marker: PhantomData,
        }
    }
}
