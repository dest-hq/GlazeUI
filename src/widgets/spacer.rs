use std::marker::PhantomData;

use crate::{core::widget::Widget, widgets::ui::build_spacer};

pub struct Spacer<App> {
    pub height: f32,
    pub width: f32,
    _marker: PhantomData<App>,
}

impl<App> Spacer<App> {
    pub fn new() -> Self {
        Self {
            height: 0.0,
            width: 0.0,
            _marker: PhantomData,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn build(self) -> Widget<App> {
        build_spacer(self)
    }
}
