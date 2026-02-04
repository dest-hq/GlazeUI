use std::marker::PhantomData;

use crate::{Margin, Widget, id::next_id, style::Style};

#[derive(Debug)]
pub struct VStack<App: 'static> {
    pub children: Vec<Widget<App>>,
    pub margin: Margin,
    pub spacing: i32,
}

impl<App> VStack<App> {
    pub fn new(children: Vec<Widget<App>>) -> Self {
        Self {
            children,
            margin: Margin::new(),
            spacing: 0,
        }
    }

    pub fn extend(mut self, children: Vec<Widget<App>>) -> Self {
        self.children = children;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn child(mut self, child: Widget<App>) -> Self {
        self.children.push(child);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn build(self) -> Widget<App> {
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
