use crate::widgets::text::TextWeight;
use taffy::Style;

/// Widget with a generic Message type
#[derive(Debug, Clone)]
pub struct Widget<Message> {
    /// Unique Id
    pub id: u64,

    /// Type of UI element
    pub element: WidgetElement<Message>,

    /// Styles
    pub style: Style,

    /// Message that will be sent on click
    pub on_click: Option<Message>,
}

impl<Message> Widget<Message> {
    /// Create a new widget
    pub fn new(id: u64, element: WidgetElement<Message>, on_click: Option<Message>) -> Self {
        Self {
            id,
            element,
            style: Style::default(),
            on_click,
        }
    }
}

/// Types of UI elements
#[derive(Debug, Clone)]
pub enum WidgetElement<Message> {
    /// A container that holds a child
    Container {
        child: Box<Widget<Message>>,
        width: f32,
        height: f32,
        color: (u8, u8, u8, u8),
        radius: f32,
    },

    Text {
        content: String,
        font_size: u32,
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
    Spacer { height: f32, width: f32 },

    /// Vertical list
    VStack {
        spacing: f32,
        children: Vec<Widget<Message>>,
    },

    /// Horizontal list
    HStack {
        spacing: f32,
        children: Vec<Widget<Message>>,
    },
}
