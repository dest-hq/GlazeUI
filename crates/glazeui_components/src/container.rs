use glazeui_core::{Node, NodeElement};
use taffy::{Size, Style, prelude::length};

use crate::ui_id::{next_id, sync_with};

// Helper to create container easier

pub fn container(child: Node) -> Container {
    Container::new(child)
}

pub struct Container {
    child: Node,
    width: f32,
    height: f32,
    color: (u8, u8, u8, u8),
    radius: f32,
}

impl Container {
    pub fn new(child: Node) -> Self {
        Self {
            child,
            width: 100.0,
            height: 50.0,
            color: (255, 255, 255, 255),
            radius: 0.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn color(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.color = (r, g, b, a);
        self
    }
    pub fn radius(mut self, corner_radius: f32) -> Self {
        self.radius = corner_radius;
        self
    }

    // Transform in Node with id
    pub fn build_with(self, id: u64) -> Node {
        sync_with(id + 1);
        let mut node = Node::new(
            id,
            NodeElement::Container {
                child: Box::new(self.child),
                width: self.width,
                height: self.height,
                color: self.color,
                radius: self.radius,
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

    // Transform in Node without id
    pub fn build(self) -> Node {
        let mut node = Node::new(
            next_id(),
            NodeElement::Container {
                child: Box::new(self.child),
                width: self.width,
                height: self.height,
                radius: self.radius,
                color: self.color,
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
