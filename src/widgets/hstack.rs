use crate::{
    core::{ui::Ui, widget::Widget},
    types::{Align, Length},
};

use crate::types::Padding;

pub struct HStack<App> {
    pub children: Vec<Widget<App>>,
    pub spacing: f32,
    pub padding: Padding,
    pub align: Option<Align>,
    pub length: Option<Length>,
}

pub struct HStackHandle<'a, App> {
    pub ui: &'a mut Ui<App>,
    pub hstack: HStack<App>,
}

impl<'a, App> HStackHandle<'a, App> {
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.hstack.spacing = spacing;
        self
    }

    pub fn children(mut self, children: Vec<Widget<App>>) -> Self {
        self.hstack.children = children;
        self
    }

    pub fn child(mut self, child: Widget<App>) -> Self {
        self.hstack.children.push(child);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.hstack.padding = padding;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.hstack.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.hstack.length = Some(length);
        self
    }

    pub fn show(self) {
        self.ui.push_hstack(self.hstack);
    }

    pub fn build(self) -> Widget<App> {
        self.ui.build_hstack(self.hstack)
    }
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::widgets::hstack::HStack::new(children)
    }};
}

impl<App> HStack<App> {
    pub fn new(children: Vec<Widget<App>>) -> Self {
        Self {
            children,
            spacing: 0.0,
            // id: None,
            padding: Padding {
                top: 0.0,
                left: 0.0,
                right: 0.0,
                bottom: 0.0,
            },
            align: None,
            length: None,
        }
    }
}
