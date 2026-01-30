use std::marker::PhantomData;

use crate::{Widget, id::next_id};

pub struct Spacer<App> {
    pub width: u32,
    pub height: u32,
    _marker: PhantomData<App>,
}

impl<App> Spacer<App> {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            _marker: PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn build(self) -> Widget<App> {
        return Widget::new(
            next_id(),
            crate::WidgetElement::Spacer {
                width: self.width,
                height: self.height,
            },
            None,
        );
    }
}
