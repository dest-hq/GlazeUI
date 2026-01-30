use std::marker::PhantomData;

use crate::{Widget, id::next_id};

#[derive(Debug)]
pub struct VStack<App: 'static> {
    pub children: Vec<Widget<App>>,
    pub spacing: i32,
}

impl<App> VStack<App> {
    pub fn new(children: Vec<Widget<App>>) -> Self {
        Self {
            children,
            spacing: 10,
        }
    }

    pub fn extend(mut self, children: Vec<Widget<App>>) -> Self {
        self.children = children;
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
        Widget {
            id: next_id(),
            element: crate::WidgetElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            on_press: None,
            _marker: PhantomData,
        }
    }
}
