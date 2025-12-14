use glaze_core::{Node, NodeElement};

// Helper to create text easier

pub fn text(content: String) -> Text {
    Text::new(content)
}

pub struct Text {
    content: String,
    size: f32,
}

impl Text {
    pub fn new(content: String) -> Self {
        Self {
            content,
            size: 16.0,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    // Transform in Node
    pub fn id(self, id: u64) -> Node {
        Node::new(
            id,
            NodeElement::Text {
                content: self.content,
                size: self.size,
            },
        )
    }
}
