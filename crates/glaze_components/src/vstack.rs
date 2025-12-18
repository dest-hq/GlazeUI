use glaze_core::{Node, NodeElement};
use taffy::{Style, prelude::length};
// Helper to create vstack easier

pub struct VStack {
    children: Vec<Node>,
    spacing: f32,
}

pub fn vstack(children: &[Node]) -> VStack {
    VStack::new(children.to_vec())
}

impl VStack {
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

    // Transform in Node with id
    pub fn build_with(self, id: u64) -> Node {
        let mut node = Node {
            id: Some(id),
            element: NodeElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: length(0.0),
                height: length(self.spacing),
            },
            ..Default::default()
        };
        node
    }

    // Transform in Node without id
    pub fn build(self) -> Node {
        let mut node = Node {
            id: None,
            element: NodeElement::VStack {
                spacing: self.spacing,
                children: self.children,
            },
            style: Style::default(),
        };
        node.style = Style {
            display: taffy::Display::Flex,
            flex_direction: taffy::FlexDirection::Column,
            gap: taffy::Size {
                width: length(0.0),
                height: length(self.spacing),
            },
            ..Default::default()
        };
        return node;
    }
}
