use crate::core::node::{Node, NodeElement};
use taffy::{Size, Style, prelude::length};

use crate::widgets::utils::ui_id::next_id;

// Helper to create spacer easier

pub struct Spacer {
    height: f32,
    width: f32,
}

#[allow(dead_code)]
pub fn spacer() -> Spacer {
    Spacer::new()
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            height: 0.0,
            width: 0.0,
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

// Transform in Node
impl From<Spacer> for Node {
    fn from(builder: Spacer) -> Node {
        let mut node = Node {
            id: next_id(),
            element: NodeElement::Spacer {
                width: builder.width,
                height: builder.height,
            },
            style: Style::default(),
        };
        node.style = Style {
            size: Size {
                width: length(builder.width),
                height: length(builder.height),
            },
            ..Default::default()
        };
        node
    }
}
