use crate::{
    core::widget::Widget,
    types::{Align, Length, Padding},
    widgets::ui::build_hstack,
};

#[macro_export]
macro_rules! hstack {
    ($($child:expr),*) => {{
        let children = vec![$($child),*];
        glazeui::widgets::hstack::HStack::new(children)
    }};
}

pub struct HStack<App> {
    pub children: Vec<Widget<App>>,
    pub spacing: f32,
    pub padding: Padding,
    pub align: Option<Align>,
    pub length: Option<Length>,
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

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn extend(mut self, children: Vec<Widget<App>>) -> Self {
        self.children = children;
        self
    }

    pub fn child(mut self, child: Widget<App>) -> Self {
        self.children.push(child);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn length(mut self, length: Length) -> Self {
        self.length = Some(length);
        self
    }

    pub fn build(self) -> Widget<App> {
        build_hstack(self)
    }
}
