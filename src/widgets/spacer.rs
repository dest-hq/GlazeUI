use std::marker::PhantomData;

use crate::core::node::{NodeElement, Widget};
use taffy::{Size, Style, prelude::length};

use crate::widgets::utils::ui_id::next_id;

pub struct Spacer<Message> {
    _marker: PhantomData<Message>,
    height: f32,
    width: f32,
}

#[allow(dead_code)]

// Helper to create spacer easier
pub fn spacer<Message>() -> Spacer<Message> {
    Spacer::new()
}

impl<Message> Spacer<Message> {
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
}

// Transform in Widget
impl<Message> From<Spacer<Message>> for Widget<Message> {
    fn from(builder: Spacer<Message>) -> Widget<Message> {
        let mut widget = Widget {
            id: next_id(),
            element: NodeElement::Spacer {
                width: builder.width,
                height: builder.height,
            },
            on_click: None,
            style: Style::default(),
        };
        widget.style = Style {
            size: Size {
                width: length(builder.width),
                height: length(builder.height),
            },
            ..Default::default()
        };
        widget
    }
}
