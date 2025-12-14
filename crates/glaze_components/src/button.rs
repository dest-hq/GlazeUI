use glaze_core::{Node, NodeElement};

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

    // Transform in Node
    pub fn id(self, id: u64) -> Node {
        Node::new(
            id,
            NodeElement::Button {
                label: self.label,
                width: self.width,
                height: self.height,
            },
        )
    }
}
