use crate::{
    core::{ui::Ui, widget::Widget},
    types::{Align, Length},
};

use crate::types::Padding;

// Helper to create vstack easier

#[derive(Debug)]
pub struct VStack<App> {
    pub children: Vec<Widget<App>>,
    pub spacing: f32,
    pub padding: Padding,
    pub align: Option<Align>,
    pub length: Option<Length>,
}

pub struct VStackHandle<'a, App> {
    pub ui: &'a mut Ui<App>,
    pub vstack: VStack<App>,
}

impl<'a, App> VStackHandle<'a, App> {
    pub fn children(mut self, children: Vec<Widget<App>>) -> Self {
        self.vstack.children = children;
        self
    }

    pub fn child(mut self, child: Widget<App>) -> Self {
        self.vstack.children.push(child);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.vstack.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.vstack.length = Some(length);
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.vstack.spacing = spacing;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.vstack.padding = padding;
        self
    }

    pub fn show(self) {
        self.ui.push_vstack(self.vstack);
    }

    pub fn build(self) -> Widget<App> {
        self.ui.build_vstack(self.vstack)
    }
}

impl<App> VStack<App> {
    pub fn new(children: Vec<Widget<App>>) -> Self {
        Self {
            children,
            spacing: 10.0,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            // id: None,
            align: None,
            length: None,
        }
    }
}
