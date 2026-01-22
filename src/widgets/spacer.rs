use std::marker::PhantomData;

use crate::core::ui::Ui;

pub struct Spacer<App> {
    pub height: f32,
    pub width: f32,
    _marker: PhantomData<App>,
}

pub struct SpacerHandler<'a, App> {
    pub ui: &'a mut Ui<App>,
    pub spacer: Spacer<App>,
}

impl<App> Spacer<App> {
    pub fn new() -> Self {
        Self {
            height: 0.0,
            width: 0.0,
            _marker: PhantomData,
        }
    }
}

impl<'a, App> SpacerHandler<'a, App> {
    pub fn width(mut self, width: f32) -> Self {
        self.spacer.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.spacer.height = height;
        self
    }

    pub fn build(self) {
        self.ui.push_spacer(self.spacer);
    }
}
