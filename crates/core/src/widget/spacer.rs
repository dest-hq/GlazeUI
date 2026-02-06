use std::marker::PhantomData;

use crate::{Widget, id::next_id, style::Style};

pub struct Spacer<M: Clone, App> {
    pub width: u32,
    pub height: u32,
    _marker_app: PhantomData<App>,
    _marker_message: PhantomData<M>,
}

impl<M: Clone, App> Spacer<M, App> {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            _marker_app: PhantomData,
            _marker_message: PhantomData,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn build(self) -> Widget<M, App> {
        // Spacer style
        let spacer_style = Style {
            width: self.width,
            height: self.height,
            ..Default::default()
        };

        Widget {
            id: next_id(),
            element: crate::WidgetElement::Spacer {},
            on_press: None,
            style: spacer_style,
            _marker: PhantomData,
        }
    }
}
