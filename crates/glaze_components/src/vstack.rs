use glaze_core::{Node, NodeElement};

// Helper to create vstack easier

pub struct VStack {
    children: Vec<Node>,
    spacing: f32,
}

pub fn vstack() -> VStack {
    VStack::new()
}

#[allow(clippy::new_without_default)]
impl VStack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    // Transform in Node
    pub fn id(self, id: u64) -> Node {
        Node {
            id,
            element: NodeElement::VStack {
                spacing: self.spacing,
            },
            children: self.children,
        }
    }
}
