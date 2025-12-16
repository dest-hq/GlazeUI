use glaze_core::{Node, NodeElement};
use taffy::{prelude::length, Style};

// Helper to create hstack easier

pub struct HStack {
    children: Vec<Node>,
    spacing: f32,
}

pub fn hstack(children: &[Node]) -> HStack {
    HStack::new(children.to_vec())
}

impl HStack {
    pub fn new(children: Vec<Node>) -> Self {
        Self {
            children,
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    // Transform in Node
    pub fn id(self, id: u64) -> Node {
        let mut node = Node {
            id,
            element: NodeElement::HStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Row,
            gap: taffy::Size {
                width: length(self.spacing),
                height: length(0.0),
            },
            ..Default::default()
        };
        node
    }
}
