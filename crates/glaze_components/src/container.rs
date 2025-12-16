use glaze_core::{Node, NodeElement};
use taffy::{prelude::length, Size, Style};

// Helper to create container easier

pub fn container(child: Node) -> Container {
    Container::new(child)
}

pub struct Container {
    child: Node,
    width: f32,
    height: f32,
}

impl Container {
    pub fn new(child: Node) -> Self {
        Self {
            child,
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

    // Transform in Node
    pub fn id(self, id: u64) -> Node {
        let mut node = Node::new(
            id,
            NodeElement::Container {
                child: Box::new(self.child),
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
