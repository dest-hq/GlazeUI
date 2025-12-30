use glazeui_core::{Node, NodeElement};
use taffy::{Size, Style, prelude::length};

// Helper to create button easier

pub fn button(label: String) -> Button {
    Button::new(label)
}

pub struct Button {
    label: String,
    width: f32,
    height: f32,
}

impl Button {
    pub fn new(label: String) -> Self {
        Self {
            label,
            width: 100.0,
            height: 50.0,
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

    // Transform in Node without id
    pub fn build(self) -> Node {
        let mut node = Node::new(
            None,
            NodeElement::Button {
                label: self.label,
                width: self.width,
                height: self.height,
            },
        );
        node.style = Style {
            size: Size {
                width: length(self.width),
                height: length(self.height),
            },

            ..Default::default()
        };
        node
    }

    // Transform in Node with id
    pub fn build_with(self, id: u64) -> Node {
        let mut node = Node::new(
            Some(id),
            NodeElement::Button {
                label: self.label,
                width: self.width,
                height: self.height,
            },
        );
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
