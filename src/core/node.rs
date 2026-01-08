use taffy::Style;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TextWeight {
    THIN,        // 100
    EXTRA_LIGHT, // 200
    LIGHT,       // 300
    NORMAL,      // 400,
    MEDIUM,      // 500
    SEMIBOLD,    // 600
    BOLD,        // 700
    EXTRA_BOLD,  // 800
    BLACK,       // 900
}

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
            id: id,
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
        color: (u8, u8, u8, u8),
        radius: f32,
    },

    Text {
        content: String,
        font_size: f32,
        line_height: f32,
        weight: TextWeight,
    },

    Input {
        content: String,
        font_size: f32,
        line_height: f32,
        weight: TextWeight,
    },

    Button {
        label: String,
        width: f32,
        height: f32,
    },

    /// Empty space
    Spacer {
        height: f32,
        width: f32,
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
