use taffy::Style;

#[derive(Debug, Clone)]
pub struct Node {
    /// Unique Id
    pub id: u64,

    /// What type of UI element that is
    pub element: NodeElement,

    // Style of element
    pub style: Style,
}

impl Node {
    /// Create a new node
    pub fn new(id: u64, element: NodeElement) -> Self {
        Self {
            id,
            element,
            style: Style::default(),
        }
    }
}

/// Types of UI elements
#[derive(Debug, Clone)]
pub enum NodeElement {
    /// A box that holds other things
    Container {
        child: Box<Node>,
        width: f32,
        height: f32,
    },

    Text {
        content: String,
        font_size: f32,
        line_height: f32,
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
        children: Vec<Node>,
    },

    // Horizontal List
    HStack {
        spacing: f32,
        children: Vec<Node>,
    },
}
