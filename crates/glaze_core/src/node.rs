#[derive(Debug, Clone)]
pub struct Node {
    /// Unique Id
    pub id: u64,

    /// What type of UI element that is
    pub element: NodeElement,

    /// Children (for containers)
    pub children: Vec<Node>,
}

impl Node {
    /// Create a new node
    pub fn new(id: u64, element: NodeElement) -> Self {
        Self {
            id,
            element,
            children: Vec::new(),
        }
    }

    /// Add a child
    pub fn push_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

/// Types of UI elements
#[derive(Debug, Clone)]
pub enum NodeElement {
    /// A box that holds other things
    Container,

    Text {
        content: String,
        size: f32,
    },

    Button {
        label: String,
        width: f32,
        height: f32,
    },

    /// Empty space
    Spacer {
        height: f32,
    },

    // Vertical List
    VStack {
        spacing: f32,
    },
}
