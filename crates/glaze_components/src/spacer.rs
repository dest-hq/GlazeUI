use glaze_core::{Node, NodeElement};
use taffy::{Size, Style, prelude::length};

// Helper to create spacer easier

pub struct Spacer {
    height: f32,
    width: f32,
}

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

    // Transform in Node
    pub fn build(self) -> Node {
        let mut node = Node {
            id: None,
            element: NodeElement::Spacer {
                width: self.width,
                height: self.height,
            },
            style: Style::default(),
        };
        node.style = Style {
            size: Size {
                width: length(self.width),
                height: length(self.height),
            },
            ..Default::default()
        };
        node
    }
}
